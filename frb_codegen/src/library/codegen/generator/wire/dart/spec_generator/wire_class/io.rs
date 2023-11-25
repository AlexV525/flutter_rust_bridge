use crate::codegen::generator::wire::dart::internal_config::{
    DartOutputClassNamePack, GeneratorWireDartInternalConfig,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::library::commands::ffigen::{ffigen, FfigenArgs};
use anyhow::ensure;

pub(crate) fn generate(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
) -> anyhow::Result<WireDartOutputCode> {
    let content = execute_ffigen(config, c_file_content)?;
    let content = postpare_modify(&content, &config.dart_output_class_name_pack);
    sanity_check(&content, &config.dart_output_class_name_pack)?;
    Ok(WireDartOutputCode::parse(&content))
}

fn execute_ffigen(
    config: &GeneratorWireDartInternalConfig,
    c_file_content: &str,
) -> anyhow::Result<String> {
    ffigen(FfigenArgs {
        c_file_content,
        dart_class_name: &config.dart_output_class_name_pack.wire_class_name,
        llvm_path: &config.llvm_path,
        llvm_compiler_opts: &config.llvm_compiler_opts,
        dart_root: &config.dart_root,
    })
}

fn postpare_modify(
    content_raw: &str,
    dart_output_class_name_pack: &DartOutputClassNamePack,
) -> String {
    let DartOutputClassNamePack {
        wire_class_name, ..
    } = &dart_output_class_name_pack;

    content_raw
        .replace(
            &format!("class {wire_class_name} {{"),
            &format!(
                "class {wire_class_name} implements BaseWire {{

                factory {wire_class_name}.fromExternalLibrary(ExternalLibrary lib) =>
                  {wire_class_name}(lib.ffiDynamicLibrary);
                "
            ),
        )
        .replace("final class DartCObject extends ffi.Opaque {}", "")
        .replace("final class _Dart_Handle extends ffi.Opaque {}", "")
        .replace("typedef WireSyncReturn = ffi.Pointer<DartCObject>;", "")
}

fn sanity_check(
    generated_dart_wire_code: &str,
    dart_output_class_name_pack: &DartOutputClassNamePack,
) -> anyhow::Result<()> {
    ensure!(
        generated_dart_wire_code.contains(&dart_output_class_name_pack.wire_class_name),
        "Nothing is generated for dart wire class. \
            Maybe you forget to put code like `mod the_generated_bridge_code;` to your `lib.rs`?",
    );
    Ok(())
}
