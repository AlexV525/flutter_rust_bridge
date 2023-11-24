use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::ir::ty::IrType;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use convert_case::{Case, Casing};
use IrType::RustOpaqueRef;

impl<'a> ApiDartGeneratorClassTrait for RustOpaqueRefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let dart_entrypoint_class_name = &self.context.config.dart_entrypoint_class_name;
        let dart_api_instance = format!("{dart_entrypoint_class_name}.instance.api");

        let dart_api_type =
            ApiDartGenerator::new(RustOpaqueRef(self.ir.clone()), self.context).dart_api_type();
        let dart_api_type_camel = dart_api_type.to_case(Case::Camel);

        Some(ApiDartGeneratedClass {
            namespace: self.ir.ident.0.namespace.clone(),
            code: format!(
                "@sealed class {dart_api_type} extends FrbOpaque {{
                    {dart_api_type}.fromRaw(int ptr, int size): super.unsafe(ptr, size);

                    @override
                    OpaqueDropFnType get dropFn => {dart_api_instance}.dropOpaque{dart_api_type};
            
                    @override
                    OpaqueShareFnType get shareFn => {dart_api_instance}.shareOpaque{dart_api_type};
            
                    @override
                    OpaqueTypeFinalizer get staticFinalizer => {dart_api_instance}.{dart_api_type_camel}Finalizer;
                }}"
            ),
            needs_freezed: false,
            ..Default::default()
        })
    }
}
