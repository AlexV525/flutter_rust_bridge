use crate::codegen::generator::api_dart::spec_generator::class::method::generate_api_methods;
use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::{
    generate_dart_comments, generate_dart_metadata,
};
use crate::library::codegen::generator::api_dart::spec_generator::base::*;

impl<'a> ApiDartGeneratorClassTrait for StructRefApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<ApiDartGeneratedClass> {
        let src = self.ir.get(self.context.ir_pack);
        let comments = generate_dart_comments(&src.comments);
        let metadata = generate_dart_metadata(&src.dart_metadata);

        let methods = generate_api_methods(&src.name, self.context);

        let constructor_postfix = TODO;

        Some(ApiDartGeneratedClass {
            namespace: src.name.namespace.clone(),
            code: if src.using_freezed() {
                self.generate_mode_freezed(src, &comments, &metadata, &methods, constructor_postfix)
            } else {
                self.generate_mode_non_freezed(
                    src,
                    &comments,
                    &metadata,
                    &methods,
                    constructor_postfix,
                )
            },
            needs_freezed: src.using_freezed(),
            ..Default::default()
        })
    }
}
