use crate::basic_code_impl;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::codegen::generator::wire::dart::internal_config::DartOutputClassNamePack;
use crate::utils::basic_code::DartBasicHeaderCode;
use itertools::Itertools;
use serde::Serialize;
use std::ops::AddAssign;

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct WireDartOutputCode {
    pub basic: DartBasicHeaderCode,
    pub body_top: String,
    pub api_body: String,
    pub api_impl_body: String,
    pub body: String,
}

basic_code_impl!(WireDartOutputCode);

impl AddAssign for WireDartOutputCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.basic += rhs.basic;
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
            basic: DartBasicHeaderCode {
                import: imports.join("\n"),
                ..Default::default()
            },
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

        let sorted_imports = self
            .import
            .split("\n")
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .sorted()
            .join("\n");

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
            "{}\n{}\n{}\n{}\n{}\n{}\n{}",
            self.file_top,
            sorted_imports,
            self.part,
            self.body_top,
            api_class_code,
            api_impl_class_code,
            self.body
        )
    }
}
