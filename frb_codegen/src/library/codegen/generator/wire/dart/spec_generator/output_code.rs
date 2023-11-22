use crate::basic_code_impl;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use std::ops::AddAssign;

#[derive(Default, Clone)]
pub(crate) struct WireDartOutputCode {
    pub import: String,
    pub part: String,
    pub body_top: String,
    pub api_body: String,
    pub api_impl_body: String,
    pub body: String,
}

basic_code_impl!(WireDartOutputCode);

impl AddAssign for WireDartOutputCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.import += &rhs.import;
        self.part += &rhs.part;
        self.body_top += &rhs.body_top;
        self.api_body += &rhs.api_body;
        self.api_impl_body += &rhs.api_impl_body;
        self.body += &rhs.body;
    }
}

impl WireDartOutputCode {
    pub fn parse(raw: &str) -> WireDartOutputCode {
        let (mut imports, mut body) = (Vec::new(), Vec::new());
        for line in raw.split('\n') {
            (if line.starts_with("import ") {
                &mut imports
            } else {
                &mut body
            })
            .push(line);
        }
        WireDartOutputCode {
            import: imports.join("\n"),
            body: body.join("\n"),
            ..Default::default()
        }
    }

    pub(crate) fn all_code(
        &self,
        target: TargetOrCommon,
        dart_output_class_name_pack: &DartOutputClassNamePack,
    ) -> String {
        let DartOutputClassNamePack {
            api_class_name,
            api_impl_class_name,
            api_impl_platform_class_name,
            wire_class_name,
            ..
        } = &dart_output_class_name_pack;
        let WireDartOutputCode {
            api_body,
            api_impl_body,
            ..
        } = &self;

        let api_class_code = if target == TargetOrCommon::Common {
            format!(
                "
                abstract class {api_class_name} extends BaseApi {{
                  {api_body}
                }}
                ",
            )
        } else {
            assert_eq!(api_body, "");
            "".to_owned()
        };

        let api_impl_class_code = if target == TargetOrCommon::Common {
            format!(
                "
                class {api_impl_class_name} extends {api_impl_platform_class_name} implements {api_class_name} {{
                  {api_impl_class_name}({{
                    super.handler,
                    required super.wire,
                    required super.generalizedFrbRustBinding,
                  }});

                  {api_impl_body}
                }}
                ",
            )
        } else {
            format!(
                "
                abstract class {api_impl_platform_class_name} extends BaseApiImpl<{wire_class_name}> {{
                  {api_impl_platform_class_name}({{
                    super.handler,
                    required super.wire,
                    required super.generalizedFrbRustBinding,
                  }});

                  {api_impl_body}
                }}
                ",
            )
        };

        format!(
            "{}\n{}\n{}\n{}\n{}\n{}",
            self.import, self.part, self.body_top, api_class_code, api_impl_class_code, self.body
        )
    }
}
