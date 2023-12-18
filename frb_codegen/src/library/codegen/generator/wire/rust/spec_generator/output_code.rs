use crate::basic_code_impl;
use crate::codegen::generator::wire::rust::spec_generator::extern_func::{ExternClass, ExternFunc};
use itertools::Itertools;
use serde::Serialize;
use std::ops::AddAssign;

#[derive(Default, Clone, Debug, Serialize)]
pub(crate) struct WireRustOutputCode {
    pub(crate) body: String,
    pub(crate) extern_funcs: Vec<ExternFunc>,
    pub(crate) extern_classes: Vec<ExternClass>,
}

basic_code_impl!(WireRustOutputCode);

impl WireRustOutputCode {
    pub(crate) fn all_code(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.body,
            (self.extern_funcs.iter().map(|func| func.generate())).join("\n"),
            (self.extern_classes.iter().map(|func| func.generate())).join("\n"),
        )
    }
}

impl AddAssign for WireRustOutputCode {
    #[inline]
    fn add_assign(&mut self, rhs: Self) {
        self.body += &rhs.body;
        self.extern_funcs.extend(rhs.extern_funcs);
        self.extern_classes.extend(rhs.extern_classes);
    }
}

impl From<ExternFunc> for WireRustOutputCode {
    fn from(value: ExternFunc) -> Self {
        vec![value].into()
    }
}

impl From<Vec<ExternFunc>> for WireRustOutputCode {
    fn from(extern_funcs: Vec<ExternFunc>) -> Self {
        Self {
            extern_funcs,
            ..Default::default()
        }
    }
}
