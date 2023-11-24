use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_imports_which_types_and_funcs_use;
use crate::codegen::generator::misc::target::{TargetOrCommon, TargetOrCommonMap};
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::codegen::generator::wire::dart::spec_generator::base::{
    WireDartGenerator, WireDartGeneratorContext,
};
use crate::codegen::generator::wire::dart::spec_generator::output_code::WireDartOutputCode;
use crate::codegen::ir::namespace::Namespace;
use crate::codegen::ir::pack::IrPackComputedCache;
use crate::library::codegen::generator::wire::dart::spec_generator::misc::ty::WireDartGeneratorMiscTrait;
use crate::utils::basic_code::DartBasicHeaderCode;
use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use itertools::Itertools;
use pathdiff::diff_paths;
use serde::Serialize;
use std::path::{Path, PathBuf};

mod api_impl_body;
mod api_impl_opaque;
pub(crate) mod ty;

#[derive(Clone, Serialize)]
pub(crate) struct WireDartOutputSpecMisc {
    pub(crate) wire_class: Acc<Vec<WireDartOutputCode>>,
    pub(crate) boilerplate: Acc<Vec<WireDartOutputCode>>,
    pub(crate) api_impl_normal_functions: Vec<WireDartOutputCode>,
    pub(crate) extra_functions: Acc<Vec<WireDartOutputCode>>,
}

pub(crate) fn generate(
    context: WireDartGeneratorContext,
    cache: &IrPackComputedCache,
    c_file_content: &str,
    api_dart_actual_output_paths: &[PathBuf],
) -> anyhow::Result<WireDartOutputSpecMisc> {
    Ok(WireDartOutputSpecMisc {
        wire_class: super::wire_class::generate(context.config, c_file_content)?,
        boilerplate: generate_boilerplate(api_dart_actual_output_paths, cache, context)?,
        api_impl_normal_functions: (context.ir_pack.funcs.iter())
            .map(|f| api_impl_body::generate_api_impl_normal_function(f, context))
            .collect::<anyhow::Result<Vec<_>>>()?,
        extra_functions: (cache.distinct_types.iter())
            .flat_map(|ty| WireDartGenerator::new(ty.clone(), context).generate_extra_functions())
            .collect(),
    })
}

fn generate_boilerplate(
    api_dart_actual_output_paths: &[PathBuf],
    cache: &IrPackComputedCache,
    context: WireDartGeneratorContext,
) -> anyhow::Result<Acc<Vec<WireDartOutputCode>>> {
    let DartOutputClassNamePack {
        entrypoint_class_name,
        api_class_name,
        api_impl_class_name,
        wire_class_name,
        ..
    } = &context.config.dart_output_class_name_pack;

    let file_top = "// ignore_for_file: unused_import, unused_element\n".to_owned();

    let mut universal_imports = generate_import_dart_api_layer(
        &context.config.dart_impl_output_path,
        api_dart_actual_output_paths,
    )?;
    universal_imports += &generate_imports_which_types_and_funcs_use(
        &Namespace::new_self_crate(file_stem(&context.config.dart_impl_output_path.io)),
        &Some(&cache.distinct_types.iter().collect_vec()),
        &None,
        context.as_api_dart_context(),
    )?;
    universal_imports += "
    import 'dart:convert';
    import 'dart:async';
    ";

    Ok(Acc {
        common: vec![WireDartOutputCode {
            header: DartBasicHeaderCode {
                file_top: file_top.clone(),
                import: format!(
                    "
                    {universal_imports}
                    import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated.dart';
                    import 'frb_generated.io.dart' if (dart.library.html) 'frb_generated.web.dart.dart';
                    "
                ),
                ..Default::default()
            },
            body_top: format!(
                r#"
                /// Main entrypoint of the Rust API
                class {entrypoint_class_name} extends BaseEntrypoint<{api_class_name}, {api_impl_class_name}, {wire_class_name}> {{
                  @internal
                  static final instance = {entrypoint_class_name}._();

                  {entrypoint_class_name}._();

                  /// Initialize flutter_rust_bridge
                  static Future<void> init({{
                    {api_class_name}? api,
                    BaseHandler? handler,
                  }}) async {{
                    await instance.initImpl(api: api, handler: handler);
                  }}
                  
                  /// Dispose flutter_rust_bridge
                  ///
                  /// The call to this function is optional, since flutter_rust_bridge (and everything else)
                  /// is automatically disposed when the app stops.
                  static void dispose() => instance.disposeImpl();

                  @override
                  ApiImplConstructor<{api_impl_class_name}, {wire_class_name}> get apiImplConstructor => {api_impl_class_name}.new;

                  @override
                  WireConstructor<{wire_class_name}> get wireConstructor => {wire_class_name}.new;

                  @override
                  String get defaultExternalLibraryStem => '{default_external_library_stem}';
                  
                  @override
                  String get defaultExternalLibraryRelativeDirectory => '{default_external_library_relative_directory}';
                }}
                "#,
                default_external_library_stem = context.config.default_external_library_stem,
                default_external_library_relative_directory = context.config.default_external_library_relative_directory,
            ),
            ..Default::default()
        }],
        io: vec![WireDartOutputCode {
            header: DartBasicHeaderCode {
                file_top: file_top.clone(),
                import: format!(
                    "
                    {universal_imports}
                    import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_io.dart';
                    import 'frb_generated.dart';
                    "
                ),
                ..Default::default()
            },
            ..Default::default()
        }],
        wasm: vec![WireDartOutputCode {
            header: DartBasicHeaderCode {
                file_top: file_top.clone(),
                import: format!(
                    "
                    {universal_imports}
                    import 'package:flutter_rust_bridge/flutter_rust_bridge_for_generated_web.dart';
                    import 'frb_generated.dart';
                    "
                    ),
                ..Default::default()
            },
            ..Default::default()
        }],
    })
}

fn file_stem(p: &Path) -> String {
    p.file_stem().unwrap().to_str().unwrap().into()
}

fn generate_import_dart_api_layer(
    dart_impl_output_path: &TargetOrCommonMap<PathBuf>,
    api_dart_actual_output_paths: &[PathBuf],
) -> anyhow::Result<String> {
    Ok(api_dart_actual_output_paths
        .iter()
        .map(|path| {
            let dir_base = (dart_impl_output_path[TargetOrCommon::Common].parent())
                .context("cannot find parent dir")?;
            let relative_path = diff_paths(path, dir_base).context("cannot find relative path")?;
            let relative_path = path_to_string(&relative_path)?;
            Ok(format!("import '{relative_path}';\n"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .join(""))
}
