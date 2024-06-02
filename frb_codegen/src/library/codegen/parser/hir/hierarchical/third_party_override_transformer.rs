use crate::codegen::ir::hir::hierarchical::crates::HirCrate;
use crate::codegen::ir::hir::hierarchical::function::HirFunction;
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::utils::crate_name::CrateName;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    if let Some(module_third_party_root) = remove_module_third_party_root(&mut pack) {
        for src in module_third_party_root.content.modules {
            transform_crate(&mut pack, src);
        }
    }
    Ok(pack)
}

fn remove_module_third_party_root(pack: &mut HirPack) -> Option<HirModule> {
    let hir_crate = pack.crates.get_mut(&CrateName::self_crate()).unwrap();
    hir_crate
        .root_module
        .content
        .remove_module_by_name(THIRD_PARTY_DIR_NAME)
}

fn transform_crate(pack: &mut HirPack, src: HirModule) -> anyhow::Result<()> {
    let crate_name = CrateName::new(
        (module_third_party_crate.meta.namespace.path().last())
            .unwrap()
            .to_string(),
    );
    if let Some(target_crate) = pack.crates.get_mut(&crate_name) {
        transform_module(target_crate.root_module, module_third_party_crate)?;
    } else {
        log::warn!(
            "Skip `{}` since there is no corresponding scanned third party crate.",
            module_third_party_crate.meta.namespace,
        );
    }

    Ok(())
}

fn transform_module(target: &mut HirModule, src: HirModule) -> anyhow::Result<()> {
    transform_module_content_function(target, src.content.functions)?;
    transform_module_content_struct_or_enum(target, src.content.structs)?;
    transform_module_content_struct_or_enum(target, src.content.enums)?;
    transform_module_content_module(target, src.content.modules)?;
    Ok(())
}

fn transform_module_content_function(
    target: &mut HirModule,
    src_content_functions: Vec<HirFunction>,
) -> anyhow::Result<()> {
    TODO;
    Ok(())
}

fn transform_module_content_struct_or_enum(
    target: &mut HirModule,
    src_content_struct_or_enums: TODO,
) -> anyhow::Result<()> {
    TODO;
    Ok(())
}

fn transform_module_content_module(
    target: &mut HirModule,
    src_content_modules: Vec<HirModule>,
) -> anyhow::Result<()> {
    for src_child_module in src_content_modules {
        let name = *src_child_module.meta.namespace.path().last().unwrap();
        if let Some(target_child_module) = target.content.get_mut_module_by_name(name) {
            transform_module(target_child_module, src_child_module)?;
        } else {
            log::warn!(
                "Skip `{}` since there is no corresponding scanned third party crate target module.",
                src_child_module.meta.namespace,
            );
        }
    }
    Ok(())
}
