use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::rust::spec_generator::base::*;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{
    ExternFunc, ExternFuncParam,
};
use crate::codegen::generator::wire::rust::spec_generator::output_code::WireRustOutputCode;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::impl_new_with_nullptr::generate_impl_new_with_nullptr_code_block;
use crate::codegen::generator::wire::rust::spec_generator::wire2api::misc::{
    generate_class_from_fields, rust_wire_type_add_prefix_or_js_value,
};
use crate::codegen::generator::wire::rust::spec_generator::wire2api::ty::WireRustGeneratorWire2apiTrait;
use crate::codegen::ir::ty::IrTypeTrait;
use std::borrow::Cow;

impl<'a> WireRustGeneratorWire2apiTrait for RustOpaqueWireRustGenerator<'a> {
    fn generate_wire2api_class(&self) -> Option<String> {
        Some(generate_class_from_fields(
            self.ir.clone(),
            self.context,
            &vec!["ptr: *const core::ffi::c_void".to_owned()],
        ))
    }

    fn generate_impl_wire2api_body(&self) -> Acc<Option<String>> {
        Acc {
            io: Some(
                "unsafe {
                flutter_rust_bridge::support::opaque_from_dart(self.ptr as _)
            }"
                .into(),
            ),
            ..Default::default()
        }
    }

    fn generate_impl_wire2api_jsvalue_body(&self) -> Option<Cow<str>> {
        Some(
            r#"
            #[cfg(target_pointer_width = "64")]
            {
                compile_error!("64-bit pointers are not supported.");
            }
    
            unsafe {
                flutter_rust_bridge::support::opaque_from_dart((self.as_f64().unwrap() as usize) as _)
            }"#
            .into(),
        )
    }

    fn generate_impl_new_with_nullptr(&self) -> Option<WireRustOutputCode> {
        Some(
            generate_impl_new_with_nullptr_code_block(
                self.ir.clone(),
                self.context,
                "Self { ptr: core::ptr::null() }",
                false,
            )
            .into(),
        )
    }

    fn generate_allocate_funcs(&self) -> Acc<WireRustOutputCode> {
        let rust_wire = self.rust_wire_type(Target::Io);

        Acc {
            io: ExternFunc {
                func_name: format!("new_{}", self.ir.safe_ident()),
                params: vec![],
                return_type: Some(format!(
                    "{}{rust_wire}",
                    self.rust_wire_modifier(Target::Io),
                )),
                body: format!("{rust_wire}::new_with_null_ptr()"),
                target: Target::Io,
            }
            .into(),
            ..Default::default()
        }
    }

    fn generate_related_funcs(&self) -> Acc<WireRustOutputCode> {
        let param_ptr = ExternFuncParam {
            name: "ptr".to_owned(),
            rust_type: "*const std::ffi::c_void".to_owned(),
            dart_type: "dynamic".into(),
        };

        let generate_impl = |target| -> WireRustOutputCode {
            vec![
                ExternFunc {
                    func_name: format!("drop_opaque_{}", self.ir.safe_ident()),
                    params: vec![param_ptr.clone()],
                    return_type: None,
                    body: format!(
                        "unsafe {{std::sync::Arc::<{}>::decrement_strong_count(ptr as _);}}",
                        self.ir.inner.rust_api_type()
                    ),
                    target,
                },
                ExternFunc {
                    func_name: format!("share_opaque_{}", self.ir.safe_ident()),
                    params: vec![param_ptr.clone()],
                    return_type: Some("*const std::ffi::c_void".to_string()),
                    body: format!(
                        "unsafe {{std::sync::Arc::<{}>::increment_strong_count(ptr as _); ptr}}",
                        self.ir.inner.rust_api_type()
                    ),
                    target,
                },
            ]
            .into()
        };
        Acc {
            io: generate_impl(Target::Io),
            wasm: generate_impl(Target::Wasm),
            ..Default::default()
        }
    }

    fn rust_wire_type(&self, target: Target) -> String {
        rust_wire_type_add_prefix_or_js_value(&self.ir, target)
    }
}
