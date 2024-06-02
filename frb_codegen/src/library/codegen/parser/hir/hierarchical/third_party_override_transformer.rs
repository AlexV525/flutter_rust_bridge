use std::fmt::Debug;

use itertools::Itertools;

use crate::codegen::ir::hir::hierarchical::function::{HirFunction, HirFunctionInner};
use crate::codegen::ir::hir::hierarchical::module::HirModule;
use crate::codegen::ir::hir::hierarchical::pack::HirPack;
use crate::codegen::ir::hir::hierarchical::struct_or_enum::HirStructOrEnum;
use crate::codegen::ir::hir::hierarchical::syn_item_struct_or_enum::SynItemStructOrEnum;
use crate::codegen::misc::THIRD_PARTY_DIR_NAME;
use crate::utils::crate_name::CrateName;

pub(super) fn transform(mut pack: HirPack) -> anyhow::Result<HirPack> {
    if let Some(module_third_party_root) = remove_module_third_party_root(&mut pack) {
        for src in module_third_party_root.content.modules {
            transform_crate(&mut pack, src)?;
        }
    }
    Ok(pack)
}

fn remove_module_third_party_root(pack: &mut HirPack) -> Option<HirModule> {
    let hir_crate = pack.get_mut_crate(&CrateName::self_crate()).unwrap();
    hir_crate
        .root_module
        .content
        .remove_module_by_name(THIRD_PARTY_DIR_NAME)
}

fn transform_crate(pack: &mut HirPack, src: HirModule) -> anyhow::Result<()> {
    let crate_name = CrateName::new((src.meta.namespace.path().last()).unwrap().to_string());
    if let Some(target_crate) = pack.get_mut_crate(&crate_name) {
        transform_module(&mut target_crate.root_module, src)?;
    } else {
        log::warn!(
            "Skip `{}` since there is no corresponding scanned third party crate.",
            src.meta.namespace,
        );
    }

    Ok(())
}

fn transform_module(target: &mut HirModule, src: HirModule) -> anyhow::Result<()> {
    transform_module_content_functions(&mut target.content.functions, src.content.functions)?;
    transform_module_content_struct_or_enums(&mut target.content.structs, src.content.structs)?;
    transform_module_content_struct_or_enums(&mut target.content.enums, src.content.enums)?;
    transform_module_content_modules(target, src.content.modules)?;
    Ok(())
}

fn transform_module_content_functions(
    target: &mut [HirFunction],
    src_content_functions: Vec<HirFunction>,
) -> anyhow::Result<()> {
    transform_module_content_general_vec(
        target,
        src_content_functions,
        |x| {
            let owner = match &x.inner {
                HirFunctionInner::Method { item_impl, .. } => {
                    Some(ty_to_string(&item_impl.self_ty))
                }
                _ => None,
            };
            (owner, x.inner.name())
        },
        |target, src| {
            target
                .inner
                .attrs_mut()
                .extend(src.inner.attrs().to_owned());
        },
    )
}

fn ty_to_string(ty: &syn::Type) -> String {
    quote::quote!(#ty).to_string()
}

fn transform_module_content_struct_or_enums<Item: SynItemStructOrEnum>(
    target: &mut [HirStructOrEnum<Item>],
    src_content_struct_or_enums: Vec<HirStructOrEnum<Item>>,
) -> anyhow::Result<()> {
    transform_module_content_general_vec(
        target,
        src_content_struct_or_enums,
        |x| x.ident.to_string(),
        |target, src| {
            target.src.attrs_mut().extend(src.src.attrs().to_owned());
        },
    )
}

fn transform_module_content_general_vec<T: Debug, K: Eq + Debug>(
    target_items: &mut [T],
    src_items: Vec<T>,
    key: impl Fn(&T) -> K,
    write: impl Fn(&mut T, T),
) -> anyhow::Result<()> {
    for src_item in src_items {
        let src_key = key(&src_item);

        let interest_target_items = target_items
            .iter_mut()
            .filter(|x| key(x) == src_key)
            .collect_vec();
        if interest_target_items.len() != 1 {
            log::warn!(
                "transform_module_content_attrable skip src_key={src_key:?}, \
                since the number of corresponding target items is not one (indeed is {}). \
                src_item={src_item:?}",
                interest_target_items.len(),
            );
            continue;
        }
        let target_item = interest_target_items.into_iter().next().unwrap();

        write(target_item, src_item);
    }
    Ok(())
}

fn transform_module_content_modules(
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
