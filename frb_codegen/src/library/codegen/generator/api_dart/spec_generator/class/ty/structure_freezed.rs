use crate::codegen::generator::api_dart::spec_generator::class::field::{
    generate_field_default, generate_field_required_modifier,
};
use crate::codegen::generator::api_dart::spec_generator::misc::generate_dart_maybe_implements_exception;
use crate::codegen::ir::ty::structure::IrStruct;
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;
use itertools::Itertools;

impl<'a> StructRefApiDartGenerator<'a> {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn generate_mode_freezed(
        &self,
        src: &IrStruct,
        comments: &str,
        metadata: &str,
        methods: &[String],
        constructor_postfix: &str,
        extra_body: &str,
        class_name: &str,
    ) -> String {
        let private_constructor = if !methods.is_empty() {
            format!("const {}._();", self.ir.ident.0.name)
        } else {
            "".to_owned()
        };

        let constructor_params =
            self.generate_mode_freezed_constructor_params(src, !methods.is_empty());
        let implements_exception = generate_dart_maybe_implements_exception(self.ir.is_exception);
        let methods_str = methods.join("\n");

        format!(
            "{comments}{metadata}class {class_name} with _${class_name} {implements_exception} {{
                {private_constructor}
                const factory {class_name}{constructor_postfix}({{{constructor_params}}}) = _{class_name};
                {methods_str}
                {extra_body}
            }}",
        )
    }

    fn generate_mode_freezed_constructor_params(
        &self,
        src: &IrStruct,
        _has_methods: bool,
    ) -> String {
        let ans = src
            .fields
            .iter()
            .map(|field| {
                let r#default =
                    generate_field_default(field, true, self.context.config.dart_enums_style);
                let required_modifier = generate_field_required_modifier(field);
                let type_str =
                    ApiDartGenerator::new(field.ty.clone(), self.context).dart_api_type();
                let name = field.name.dart_style();
                format!("{default} {required_modifier} {type_str} {name},")
            })
            .collect_vec();

        ans.join("")
    }
}
