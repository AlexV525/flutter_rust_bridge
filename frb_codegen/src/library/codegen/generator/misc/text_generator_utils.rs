use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::misc::target::TargetOrCommon;
use crate::utils::basic_code::BasicCode;
use strum::IntoEnumIterator;

pub(crate) fn section_header_comment<T: BasicCode>(
    section_name: &str,
    item: &Acc<Vec<T>>,
) -> Acc<Vec<T>> {
    item.map_ref(|x, _target| {
        vec![T::new_body(
            if x.iter().all(|x| x.body().trim().is_empty()) {
                "".to_owned()
            } else {
                section_header_comment_raw(section_name)
            },
        )]
    })
}

pub(crate) fn section_header_comment_raw(section_name: &str) -> String {
    format!("\n\n// Section: {section_name}\n\n")
}

pub(crate) fn generate_text_respecting_wasm_flag(
    raw: Acc<String>,
    wasm_enabled: bool,
) -> Acc<Option<String>> {
    raw.map(|value, target| (target != TargetOrCommon::Wasm || wasm_enabled).then_some(value))
}
