use crate::commands::BindgenRustToDartArg;

use crate::ir::IrTypeTrait;
use crate::others::{
    extract_dart_wire_content, modify_dart_wire_content, sanity_check, DartBasicCode,
    DUMMY_WIRE_CODE_FOR_BINDGEN, EXTRA_EXTERN_FUNC_NAMES,
};
use crate::utils::misc::{is_multi_blocks_case, with_changed_file, ExtraTraitForVec, ShareMode};
use crate::{command_run, commands, ensure_tools_available, generator, ir, Opts};
use convert_case::{Case, Casing};
use itertools::Itertools;
use log::info;
use pathdiff::diff_paths;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;

pub(crate) fn generate_dart_code(
    config: &Opts,
    all_configs: &[Opts],
    ir_file: &ir::IrFile,
    generated_rust: generator::rust::Output,
    all_symbols: &[String],
) -> crate::Result {
    let dart_root = config.dart_root_or_default();
    ensure_tools_available(&dart_root, config.skip_deps_check)?;

    info!("Phase: Generating Dart bindings for Rust");
    // phase-step1: generate (temporary) c file(s)
    let temp_dart_wire_file = tempfile::NamedTempFile::new()?;
    let temp_bindgen_c_output_file = tempfile::Builder::new().suffix(".h").tempfile()?;
    let mut exclude_symbols = generated_rust.get_exclude_symbols(all_symbols, ir_file);
    let mut extra_forward_declarations = Vec::new();
    //↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓refine exclude_symbols↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓↓
    match ir_file.shared {
        ShareMode::Unique => {
            // 1. refine `exclude_symbols`
            for c in all_configs {
                if c.block_index == config.block_index {
                    continue;
                }
                let (wire_types, wire_funcs) = get_wire_types_funcs_for_c_file(c, all_configs);
                exclude_symbols.extend(wire_types);
                exclude_symbols.extend(wire_funcs);
            }

            // 2. refine `extra_forward_declarations`
            let x = ir_file.get_shared_type_names(
                true,
                Some(|each: &ir::IrType| each.is_struct() || each.is_list(false)),
            );
            let extra_forward_declaration = x
                .iter()
                .map(|each| format!("typedef struct wire_{each} wire_{each}"))
                .collect::<Vec<_>>();

            extra_forward_declarations.extend(extra_forward_declaration);
        }
        ShareMode::Shared => {
            for c in all_configs {
                if c.shared == ShareMode::Shared {
                    continue;
                }
                let (wire_types, wire_funcs) = get_wire_types_funcs_for_c_file(c, all_configs);
                exclude_symbols.extend(wire_types);
                exclude_symbols.extend(wire_funcs);
            }
        }
    }
    //↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑refine exclude_symbols↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑↑
    let c_struct_names = ir_file.get_c_struct_names(all_configs);
    with_changed_file(
        &config.rust_output_path,
        DUMMY_WIRE_CODE_FOR_BINDGEN,
        || {
            commands::bindgen_rust_to_dart(
                BindgenRustToDartArg {
                    rust_crate_dir: &config.rust_crate_dir,
                    c_output_path: temp_bindgen_c_output_file
                        .path()
                        .as_os_str()
                        .to_str()
                        .unwrap(),
                    dart_output_path: temp_dart_wire_file.path().as_os_str().to_str().unwrap(),
                    dart_class_name: &config.dart_wire_class_name(),
                    c_struct_names,
                    exclude_symbols,
                    extra_forward_declarations,
                    llvm_install_path: &config.llvm_path[..],
                    llvm_compiler_opts: &config.llvm_compiler_opts,
                },
                &dart_root,
            )
            .map_err(Into::into)
        },
    )?;

    let effective_func_names = [
        generated_rust.extern_func_names,
        EXTRA_EXTERN_FUNC_NAMES.to_vec(),
    ]
    .concat();

    for (i, each_path) in config.c_output_paths.iter().enumerate() {
        let c_dummy_code =
            generator::c::generate_dummy(config, all_configs, &effective_func_names, i);
        fs::create_dir_all(Path::new(each_path).parent().unwrap())?;
        fs::write(
            each_path,
            fs::read_to_string(&temp_bindgen_c_output_file)? + "\n" + &c_dummy_code,
        )?;
    }

    // phase-step2: generate raw dart code instance from the c file
    let generated_dart_wire_code_raw = fs::read_to_string(temp_dart_wire_file)?;
    let generated_dart_wire = extract_dart_wire_content(&modify_dart_wire_content(
        &generated_dart_wire_code_raw,
        &config.dart_wire_class_name(),
        ir_file,
    ));
    sanity_check(&generated_dart_wire.body, &config.dart_wire_class_name())?;

    // phase-step3: compose dart codes and write to file
    let generated_dart = ir_file.generate_dart(config, all_configs, &generated_rust.wasm_exports);
    let generated_dart_decl_all = &generated_dart.decl_code;
    let generated_dart_impl_io_wire = &generated_dart.impl_code.io + &generated_dart_wire;

    let dart_output_paths = config.get_dart_output_paths();
    let dart_output_dir = Path::new(&dart_output_paths.base_path).parent().unwrap();
    fs::create_dir_all(dart_output_dir)?;

    if let Some(dart_decl_output_path) = &config.dart_decl_output_path {
        write_dart_decls(
            config,
            dart_decl_output_path,
            dart_output_dir,
            &generated_dart,
            generated_dart_decl_all,
            &generated_dart_impl_io_wire,
        )?;
    } else if config.wasm_enabled {
        fs::write(
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude
                + generated_dart_decl_all
                + &generated_dart.impl_code.common)
                .to_text(),
        )?;
        fs::write(
            &dart_output_paths.io_path,
            (&generated_dart.file_prelude + &generated_dart_impl_io_wire).to_text(),
        )?;
        fs::write(
            &dart_output_paths.wasm_path,
            (&generated_dart.file_prelude + &generated_dart.impl_code.wasm).to_text(),
        )?;
    } else {
        let mut out = generated_dart.file_prelude
            + generated_dart_decl_all
            + &generated_dart.impl_code.common
            + &generated_dart_impl_io_wire;
        out.import = out.import.lines().unique().join("\n");
        fs::write(&dart_output_paths.base_path, out.to_text())?;
    }

    info!("Phase: Running build_runner");
    let dart_root = &config.dart_root;
    if generated_dart.needs_freezed && config.build_runner {
        let dart_root = dart_root
            .as_ref()
            .ok_or(crate::config::Error::FailedInferDartRoot)?;
        commands::build_runner(dart_root)?;
    }

    info!("Phase: Formatting Dart code");
    command_run!(
        commands::format_dart[config.dart_format_line_length],
        &dart_output_paths.base_path,
        ?config.dart_decl_output_path,
        (
            config.wasm_enabled,
            dart_output_paths.wasm_path,
            dart_output_paths.io_path,
        ),
        (
            generated_dart.needs_freezed && config.build_runner,
            config.dart_freezed_path(),
        )
    )?;

    Ok(())
}

fn get_wire_types_funcs_for_c_file(
    config: &Opts,
    all_configs: &[Opts],
) -> (Vec<String>, Vec<String>) {
    let ir_file = config.get_ir_file(all_configs).unwrap();
    let types = ir_file.distinct_types(true, true, all_configs);

    // 1. wire_types
    let wire_types = types
        .iter()
        .map(|each| {
            if each.is_list(true) {
                format!("wire_{}", each.safe_ident())
            } else {
                format!("wire_{}", each.safe_ident().to_case(Case::Pascal))
            }
        })
        .collect::<Vec<_>>();
    let funcs = ir_file.funcs(true).into_iter().collect::<Vec<_>>();

    // 2. wire_funcs
    let wire_funcs = funcs
        .iter()
        .map(|each| format!("wire_{}", each.name))
        .collect::<Vec<_>>();

    (wire_types, wire_funcs)
}

use std::cell::RefCell;
thread_local!(static HAS_GENERATED_DART_DECL_FILE: RefCell<bool> = RefCell::new(false));

fn write_dart_decls(
    config: &Opts,
    dart_decl_output_path: &str,
    dart_output_dir: &Path,
    generated_dart: &crate::generator::dart::Output,
    generated_dart_decl_all: &DartBasicCode,
    generated_dart_impl_io_wire: &DartBasicCode,
) -> crate::Result {
    // be it single or multi block(s) case, remove the definition file at first
    HAS_GENERATED_DART_DECL_FILE.with(|data| {
        let mut flag = data.borrow_mut();
        if !(*flag) {
            std::fs::remove_file(dart_decl_output_path).unwrap_or_default();
            *flag = true;
        }
    });

    // get essential import content
    let common_import = DartBasicCode {
        import: if config.wasm_enabled {
            format!(
                "import '{}' if (dart.library.html) '{}';",
                config
                    .dart_io_output_path()
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap(),
                config
                    .dart_wasm_output_path()
                    .file_name()
                    .and_then(OsStr::to_str)
                    .unwrap(),
            )
        } else {
            "".into()
        },
        ..Default::default()
    };

    // if file of `dart_decl_output_path` existed, append content but not create
    let mut file = fs::OpenOptions::new()
        .append(true) // essential for multi-blocks case
        .create(true)
        .open(dart_decl_output_path)
        .unwrap_or_else(|_| panic!("{}", "can't open path\"{dart_decl_output_path}\""));
    std::io::Write::write_all(
        &mut file,
        (&generated_dart.file_prelude + &common_import + generated_dart_decl_all)
            .to_text()
            .as_bytes(),
    )?;

    // erase duplicated lines for multi-blocks case, like the redundant import statements and class definitions
    // NOTE: since dart file with syntax error would make the whole generation stuck,
    // the refinement MUST be done at the end after EACH dart block generation.
    if is_multi_blocks_case(None) {
        let mut contents =
            std::fs::read_to_string(dart_decl_output_path).expect("Unable to read file");
        remove_dupilicated_prehead_and_imports(&mut contents);
        fs::write(dart_decl_output_path, contents)?;
    }

    // refine import for other dart files
    let impl_import_decl = DartBasicCode {
        import: format!(
            "import \"{}\";",
            diff_paths(dart_decl_output_path, dart_output_dir)
                .unwrap()
                .to_str()
                .unwrap()
        ),
        ..Default::default()
    };
    let dart_output_paths = config.get_dart_output_paths();
    if config.wasm_enabled {
        fs::write(
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart.impl_code.common)
                .to_text(),
        )?;
        fs::write(
            dart_output_paths.io_path,
            (&generated_dart.file_prelude + &impl_import_decl + generated_dart_impl_io_wire)
                .to_text(),
        )?;
        fs::write(
            dart_output_paths.wasm_path,
            (&generated_dart.file_prelude + &impl_import_decl + &generated_dart.impl_code.wasm)
                .to_text(),
        )?;
    } else {
        fs::write(
            &dart_output_paths.base_path,
            (&generated_dart.file_prelude
                + &impl_import_decl
                + &generated_dart.impl_code.common
                + generated_dart_impl_io_wire)
                .to_text(),
        )?;
    }

    Ok(())
}

fn remove_dupilicated_prehead_and_imports(contents: &mut String) {
    let mut lines = contents.lines().collect::<Vec<_>>();
    let mut correct_imports: Vec<String> = Vec::new();
    let mut lines_to_delete = Vec::new();
    let mut check_import_surfix_next_time = false;
    for (i, line) in lines.iter().enumerate() {
        let trimmed_line = line.trim();
        if !check_import_surfix_next_time {
            if trimmed_line.starts_with("part ") || trimmed_line.starts_with("// ") {
                if !correct_imports.contains(&String::from(trimmed_line)) {
                    correct_imports.push(trimmed_line.into());
                }
                lines_to_delete.push(i);
            } else if trimmed_line.starts_with("import ") {
                if trimmed_line.ends_with(';') {
                    if !correct_imports.contains(&String::from(trimmed_line)) {
                        correct_imports.push(trimmed_line.into());
                    }
                } else {
                    check_import_surfix_next_time = true;
                }
                lines_to_delete.push(i);
            }
        } else {
            // check current line with previous `import` line together
            let j = lines_to_delete.last().unwrap();
            let new_import_line = format!("{} {}", lines[*j].trim(), trimmed_line);

            check_import_surfix_next_time = !trimmed_line.ends_with(';');

            if !correct_imports.contains(&new_import_line) {
                correct_imports.push(new_import_line);
                if !check_import_surfix_next_time {
                    correct_imports = correct_imports.find_uniques();
                }
            }
            lines_to_delete.push(i);
        }
    }

    // Sort the indices in descending to prevent issues with subsequent removals
    lines_to_delete.sort_by(|a, b| b.cmp(a));
    for i in lines_to_delete {
        lines.remove(i);
    }

    // add compact import at file top place
    correct_imports.sort();
    // this sort is essential for putting `part ...` after `import ...`
    let imports_lines = correct_imports.join("\n");
    *contents = lines.join("\n");
    *contents = format!("{}\n{}", imports_lines, contents);
}
