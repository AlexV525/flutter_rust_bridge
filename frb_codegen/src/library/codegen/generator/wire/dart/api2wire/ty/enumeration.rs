use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::wire::dart::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::dart::base::*;
use crate::codegen::ir::ty::enumeration::{IrVariant, IrVariantKind};
use crate::commands::command_runner::call_shell;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireDartGeneratorApi2wireTrait for EnumRefWireDartGenerator<'a> {
    fn api2wire_body(&self) -> Acc<Option<String>> {
        let variants = (self.ir.get(self.context.ir_pack).variants())
            .iter()
            .enumerate()
            .map(|(idx, variant)| generate_api2wire_body_variant(idx, &variant))
            .join("\n");

        Acc {
            wasm: Some(format!(
                "{variants}

                throw Exception('unreachable');"
            )),
            ..Default::default()
        }
    }

    fn api_fill_to_wire_body(&self) -> Option<String> {
        Some(
            self.ir
                .get(self.context.ir_pack)
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| self.generate_api_fill_to_wire_body_variant(idx, variant))
                .join("\n"),
        )
    }
}

impl<'a> EnumRefWireDartGenerator<'a> {
    fn generate_api_fill_to_wire_body_variant(&self, index: usize, variant: &IrVariant) -> String {
        let ident = &self.ir.ident.0;
        let wrapper_name = &variant.wrapper_name.raw;
        let variant_name = &variant.name.raw;

        let (stmt_prepare, stmt_postpare) = match &variant.kind {
            IrVariantKind::Value => ("".to_owned(), "".to_owned()),
            IrVariantKind::Struct(st) => {
                let pre_field = st
                    .fields
                    .iter()
                    .map(|field| {
                        format!(
                            "var pre_{} = api2wire_{}(apiObj.{});",
                            field.name.rust_style(),
                            field.ty.safe_ident(),
                            field.name.dart_style()
                        )
                    })
                    .join("\n");

                let stmt_set_kind =
                    format!("wireObj.kind = inner.inflate_{ident}_{variant_name}();");

                let r = format!("wireObj.kind.ref.{variant_name}.ref");
                let body = st
                    .fields
                    .iter()
                    .map(|field| {
                        format!("{r}.{name} = pre_{name};", name = field.name.rust_style(),)
                    })
                    .join("\n");

                (pre_field, stmt_set_kind + &body)
            }
        };

        format!(
            "if (apiObj is {wrapper_name}) {{
                {stmt_prepare}wireObj.tag = {index};{stmt_postpare}
                return;
            }}",
        )
    }
}

fn generate_api2wire_body_variant(index: usize, variant: &&IrVariant) -> String {
    let fields = match &variant.kind {
        IrVariantKind::Value => vec![],
        IrVariantKind::Struct(st) => (st.fields)
            .iter()
            .map(|field| {
                format!(
                    ",api2wire_{}(raw.{})",
                    field.ty.safe_ident(),
                    field.name.dart_style()
                )
            })
            .collect(),
    }
    .join("");
    format!(
        "if (raw is {variant}) {{
            return [{index} {fields}];
        }}",
        variant = variant.wrapper_name.rust_style(),
    )
}
