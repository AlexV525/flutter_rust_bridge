use crate::codegen::generator::api_dart::spec_generator::base::{
    ApiDartGenerator, ApiDartGeneratorContext,
};
use crate::codegen::mir::annotation::MirDartAnnotation;
use crate::codegen::mir::comment::MirComment;
use crate::codegen::mir::func::MirFunc;
use crate::codegen::mir::import::MirDartImport;
use crate::codegen::mir::namespace::Namespace;
use crate::codegen::mir::pack::DistinctTypeGatherer;
use crate::codegen::mir::ty::MirType;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use crate::library::codegen::mir::ty::MirTypeTrait;
use crate::utils::path_utils::path_to_string;
use anyhow::Context;
use itertools::Itertools;
use pathdiff::diff_paths;

/// A trailing newline is included if comments is not empty.
pub(crate) fn generate_dart_comments(comments: &[MirComment]) -> String {
    let mut comments = comments
        .iter()
        .map(|comment| comment.0.clone())
        .collect_vec()
        .join("\n");
    if !comments.is_empty() {
        comments.push('\n');
    }
    comments
}

pub(crate) fn generate_dart_metadata(metadata: &[MirDartAnnotation]) -> String {
    let mut metadata = metadata
        .iter()
        .map(|it| match &it.library {
            Some(MirDartImport {
                alias: Some(alias), ..
            }) => format!("@{}.{}", alias, it.content),
            _ => format!("@{}", it.content),
        })
        .collect_vec()
        .join("\n");
    if !metadata.is_empty() {
        metadata.push('\n');
    }
    metadata
}

pub(crate) fn generate_dart_maybe_implements_exception(is_exception: bool) -> &'static str {
    if is_exception {
        "implements FrbException"
    } else {
        ""
    }
}

pub(crate) fn generate_imports_which_types_and_funcs_use(
    current_file_namespace: &Namespace,
    seed_types: &Option<&Vec<&MirType>>,
    seed_funcs: &Option<&Vec<&MirFunc>>,
    context: ApiDartGeneratorContext,
) -> anyhow::Result<String> {
    let interest_types = {
        let mut gatherer = DistinctTypeGatherer::new();
        if let Some(types) = seed_types {
            (types.iter()).for_each(|x| x.visit_types(&mut |ty| gatherer.add(ty), context.ir_pack));
        }
        if let Some(funcs) = seed_funcs {
            (funcs.iter()).for_each(|x| x.visit_types(&mut |ty| gatherer.add(ty), context.ir_pack));
        }
        gatherer.gather()
    };

    let import = interest_types
        .iter()
        .map(|ty| generate_imports_from_ty(ty, current_file_namespace, context))
        .collect::<anyhow::Result<Vec<_>>>()?
        .iter()
        .join("");

    Ok(import)
}

fn generate_imports_from_ty(
    ty: &MirType,
    current_file_namespace: &Namespace,
    context: ApiDartGeneratorContext,
) -> anyhow::Result<String> {
    let import_ty_itself = if let Some(ty_namespace) = ty.self_namespace() {
        if &ty_namespace != current_file_namespace {
            let path_diff = diff_paths(
                ty_namespace.to_pseudo_io_path("dart"),
                (current_file_namespace.to_pseudo_io_path("dart").parent()).unwrap(),
            )
            .context("cannot diff path")?;
            format!(
                "import '{}';\n",
                path_to_string(&path_diff).unwrap().replace('\\', "/")
            )
        } else {
            "".to_owned()
        }
    } else {
        "".to_owned()
    };

    let import_extra = ApiDartGenerator::new(ty.clone(), context)
        .dart_import()
        .unwrap_or_default();

    Ok(import_ty_itself + &import_extra)
}
