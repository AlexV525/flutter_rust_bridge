use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::Target;
use crate::codegen::generator::wire::rust::base::*;
use crate::codegen::generator::wire::rust::wire2api::extern_func::CodeWithExternFunc;
use crate::codegen::generator::wire::rust::wire2api::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::wire2api::misc::generate_class_from_fields;
use crate::codegen::generator::wire::rust::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use crate::library::codegen::generator::wire::rust::misc::ty::WireRustGeneratorMiscTrait;
use itertools::Itertools;

impl<'a> WireRustGeneratorWire2apiTrait for StructRefWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        let s = self.ir.get(self.context.ir_pack);
        Some(generate_class_from_fields(
            self.ir.clone(),
            self.context,
            &s.fields
                .iter()
                .map(|field| {
                    let field_generator = WireRustGenerator::new(field.ty.clone(), self.context);
                    format!(
                        "{}: {}{}",
                        field.name.rust_style(),
                        field_generator.rust_wire_modifier(Target::Io),
                        field_generator.rust_wire_type(Target::Io)
                    )
                })
                .collect_vec(),
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        let api_struct = self.ir.get(self.context.ir_pack);
        let fields: Acc<Vec<_>> = api_struct
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                let field_name = field.name.rust_style();
                let field_ = if api_struct.is_fields_named {
                    format!("{field_name}: ")
                } else {
                    String::new()
                };

                Acc {
                    wasm: format!("{field_} self_.get({idx}).wire2api()"),
                    io: format!("{field_} self.{field_name}.wire2api()"),
                    ..Default::default()
                }
            })
            .collect();

        let (left, right) = api_struct.brackets_pair();
        let rust_api_type = self.ir.rust_api_type();
        Acc {
            io: Some(format!(
                "
                {rust_api_type}{left}{fields}{right}
                ",
                fields = fields.io.join(","),
            )),
            wasm: Some(format!(
                "
                let self_ = self.dyn_into::<JsArray>().unwrap();
                assert_eq!(self_.length(), {len}, \"Expected {len} elements, got {{}}\", self_.length());
                {rust_api_type}{left}{fields}{right}
                ",
                fields = fields.wasm.join(","),
                len = api_struct.fields.len(),
            )),
            ..Default::default()
        }
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<CodeWithExternFunc> {
        let src = self.ir.get(self.context.ir_pack);

        let body = {
            src.fields
                .iter()
                .map(|field| {
                    format!(
                        "{}: {},",
                        field.name.rust_style(),
                        if WireRustGenerator::new(field.ty.clone(), self.context)
                            .rust_wire_is_pointer(Target::Io)
                        {
                            "core::ptr::null_mut()".to_owned()
                        } else if matches!(field.ty, IrType::RustOpaque(_) | IrType::DartOpaque(_))
                        {
                            format!(
                                "{}::new_with_null_ptr()",
                                WireRustGenerator::new(field.ty.clone(), self.context)
                                    .rust_wire_type(Target::Io)
                            )
                        } else {
                            "Default::default()".to_owned()
                        }
                    )
                })
                .collect_vec()
                .join("\n")
        };

        Some(
            generate_impl_new_with_nullptr_code_block(
                self.ir.clone(),
                self.context,
                &format!("Self {{ {body} }}"),
                true,
            )
            .into(),
        )
    }
}
