use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::misc::ty::WireRustGeneratorMiscTrait;
use crate::codegen::ir::ty::enumeration::IrVariantKind;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorMiscTrait for EnumRefWireRustGenerator<'a> {
    fn wrapper_struct_name(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        src.wrapper_name.as_ref().map(|x| x.name).to_owned()
    }

    fn generate_static_checks(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        src.wrapper_name.as_ref()?;

        let branches = src
            .variants()
            .iter()
            .map(|variant| match &variant.kind {
                IrVariantKind::Value => format!("{}::{} => {{}}", src.name, &variant.name),
                IrVariantKind::Struct(s) => {
                    let pattern = s
                        .fields
                        .iter()
                        .map(|field| field.name.rust_style().to_owned())
                        .collect_vec();
                    let pattern = if s.is_fields_named {
                        format!("{}::{} {{ {} }}", src.name, variant.name, pattern.join(","))
                    } else {
                        format!("{}::{}({})", src.name, &variant.name, pattern.join(","))
                    };

                    let checks = s
                        .fields
                        .iter()
                        .map(|field| {
                            format!(
                                "let _: {} = {};\n",
                                field.ty.rust_api_type(),
                                field.name.rust_style(),
                            )
                        })
                        .collect_vec();

                    format!("{} => {{ {} }}", pattern, checks.join(""))
                }
            })
            .collect_vec();

        Some(format!(
            "match None::<{}>.unwrap() {{ {} }}",
            src.name,
            branches.join(","),
        ))
    }

    fn generate_imports(&self) -> Option<Vec<String>> {
        let api_enum = self.ir.get(self.context.ir_pack);
        Some(vec![format!("use {};", api_enum.path.join("::"))])
    }
}
