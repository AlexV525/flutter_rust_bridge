use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::{Target, TargetOrCommon};
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::extern_func::{
    CodeWithExternFunc, ExternFunc,
};
use crate::codegen::generator::wire::rust::wire2api::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::field::IrField;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrEnumMode, IrVariant, IrVariantKind};
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::wire::rust::info::WireRustGeneratorInfoTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorWire2apiTrait for EnumRefWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        let src = self.ir.get(self.context.ir_pack);
        if src.mode == IrEnumMode::Simple {
            return None;
        }

        let variants = src.variants();

        let variant_structs = variants
            .iter()
            .map(|variant| self.generate_wire2api_class_variant(&variant))
            .join("\n\n");

        let union_fields = variants
            .iter()
            .map(|variant| format!("{0}: *mut wire_{1}_{0},", variant.name.raw, self.ir.ident.0))
            .join("\n");

        let rust_wire_type = WireRustGenerator::new(self.ir.clone().into(), self.context.clone())
            .rust_wire_type(Target::Io);

        Some(format!(
            "#[repr(C)]
            #[derive(Clone)]
            pub struct {rust_wire_type} {{ tag: i32, kind: *mut {name}Kind }}

            #[repr(C)]
            pub union {name}Kind {{
                {union_fields}
            }}

            {variant_structs}",
            name = self.ir.ident.0,
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let enu = self.ir.get(self.context.ir_pack);
        Acc::new(|target| {
            if matches!(target, TargetOrCommon::Common) {
                return None;
            }

            let wasm = target == TargetOrCommon::Wasm;
            let variants = enu
                .variants()
                .iter()
                .enumerate()
                .map(|(idx, variant)| {
                    generate_impl_wire2api_body_variant(enu, target, idx, variant)
                })
                .join("\n");

            Some(format!(
                "{}
                match self{} {{
                    {}
                    _ => unreachable!(),
                }}",
                if wasm {
                    "let self_ = self.unchecked_into::<JsArray>();"
                } else {
                    ""
                },
                if wasm {
                    "_.get(0).unchecked_into_f64() as _"
                } else {
                    ".tag"
                },
                variants,
            ))
        })
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<CodeWithExternFunc> {
        let src = self.ir.get(self.context.ir_pack);

        let inflators = src
            .variants()
            .iter()
            .filter_map(|variant| self.generate_impl_new_with_nullptr_variant(&variant))
            .collect_vec();

        Some(CodeWithExternFunc {
            code: generate_impl_new_with_nullptr_code_block(
                self.ir.clone(),
                &self.context,
                "Self { tag: -1, kind: core::ptr::null_mut() }",
                true,
            ),
            extern_funcs: inflators,
        })
    }
}

impl<'a> EnumRefWireRustGenerator<'a> {
    fn generate_wire2api_class_variant(&self, variant: &IrVariant) -> String {
        let fields = match &variant.kind {
            IrVariantKind::Value => vec![],
            IrVariantKind::Struct(s) => s
                .fields
                .iter()
                .map(|field| {
                    let field_generator =
                        WireRustGenerator::new(field.ty.clone(), self.context.clone());
                    format!(
                        "{}: {}{},",
                        field.name.rust_style(),
                        field_generator.rust_wire_modifier(Target::Io),
                        field_generator.rust_wire_type(Target::Io)
                    )
                })
                .collect(),
        };
        format!(
            "#[repr(C)]
            #[derive(Clone)]
            pub struct wire_{}_{} {{ {} }}",
            self.ir.ident.0,
            variant.name.raw,
            fields.join("\n")
        )
    }

    fn generate_impl_new_with_nullptr_variant(&self, variant: &IrVariant) -> Option<ExternFunc> {
        let typ = format!("{}_{}", self.ir.ident.0, variant.name.raw);
        let body = if let IrVariantKind::Struct(st) = &variant.kind {
            st.fields
                .iter()
                .map(|field| self.generate_impl_new_with_nullptr_variant_field(field))
                .collect_vec()
        } else {
            return None;
        };

        Some(ExternFunc {
            func_name: format!("inflate_{typ}"),
            params: vec![],
            return_type: Some(format!("*mut {}Kind", self.ir.ident.0)),
            body: format!(
                "support::new_leak_box_ptr({}Kind {{
                    {}: support::new_leak_box_ptr({} {{
                        {}
                    }})
                }})",
                self.ir.ident.0,
                variant.name.rust_style(),
                format_args!("wire_{typ}"),
                body.join(",")
            ),
            target: Target::Io,
        })
    }

    fn generate_impl_new_with_nullptr_variant_field(&self, field: &IrField) -> String {
        let ty_generator = WireRustGenerator::new(field.ty.clone(), self.context.clone());

        let init = if ty_generator.rust_wire_is_pointer(Target::Io) {
            "core::ptr::null_mut()".to_owned()
        } else if matches!(field.ty, IrType::RustOpaque(_) | IrType::DartOpaque(_)) {
            format!(
                "{}::new_with_null_ptr()",
                ty_generator.rust_wire_type(Target::Io)
            )
        } else {
            "Default::default()".to_owned()
        };

        format!("{field_name}: {init}", field_name = field.name.rust_style())
    }
}

fn generate_impl_wire2api_body_variant(
    enu: &IrEnum,
    target: TargetOrCommon,
    idx: usize,
    variant: &IrVariant,
) -> String {
    match &variant.kind {
        IrVariantKind::Value => {
            format!("{} => {}::{},", idx, enu.name, variant.name.raw)
        }
        IrVariantKind::Struct(st) => {
            let fields = st
                .fields
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    let field_name = field.name.rust_style();
                    let field_ = if st.is_fields_named {
                        format!("{field_name}: ")
                    } else {
                        String::new()
                    };

                    if target != TargetOrCommon::Wasm {
                        format!("{field_} ans.{field_name}.wire2api()")
                    } else {
                        format!("{field_} self_.get({}).wire2api()", idx + 1)
                    }
                })
                .join(",");

            let (left, right) = st.brackets_pair();
            let enum_name = &enu.name;
            let variant_name = &variant.name.raw;

            if target == TargetOrCommon::Wasm {
                format!("{idx} => {{ {enum_name}::{variant_name}{left}{fields}{right} }},")
            } else {
                format!(
                    "{idx} => unsafe {{
                        let ans = support::box_from_leak_ptr(self.kind);
                        let ans = support::box_from_leak_ptr(ans.{variant_name});
                        {enum_name}::{variant_name}{left}{fields}{right}
                    }}",
                )
            }
        }
    }
}
