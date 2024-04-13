use crate::codegen::generator::api_dart::spec_generator::class::ApiDartGeneratedClass;
use crate::codegen::generator::api_dart::spec_generator::misc::generate_dart_comments;
use crate::codegen::ir::ty::enumeration::{IrEnum, IrVariant};
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::utils::dart_keywords::make_string_keyword_safe;
use itertools::Itertools;
use crate::codegen::generator::api_dart::spec_generator::class::method::generate_api_methods;
use crate::codegen::generator::api_dart::spec_generator::class::misc::generate_class_extra_body;

impl<'a> EnumRefApiDartGenerator<'a> {
    pub(crate) fn generate_mode_simple(&self, src: &IrEnum, extra_body: &str) -> Option<ApiDartGeneratedClass> {
        let comments = generate_dart_comments(&src.comments);

        let variants = src
            .variants()
            .iter()
            .map(|variant| self.generate_mode_simple_variant(variant))
            .collect_vec()
            .join("\n");

        let name = &self.ir.ident.0.name;

        Some(ApiDartGeneratedClass {
            namespace: src.name.namespace.clone(),
            class_name: name.clone(),
            code: format!(
                "{comments}enum {name} {{
                    {variants}
                    {extra_body}
                }}",
            ),
            needs_freezed: false,
            header: Default::default(),
        })
    }

    fn generate_mode_simple_variant(&self, variant: &IrVariant) -> String {
        let variant_name = if self.context.config.dart_enums_style {
            make_string_keyword_safe(variant.name.dart_style())
        } else {
            variant.name.rust_style().to_string()
        };

        format!(
            "{}{},",
            generate_dart_comments(&variant.comments),
            variant_name
        )
    }
}
