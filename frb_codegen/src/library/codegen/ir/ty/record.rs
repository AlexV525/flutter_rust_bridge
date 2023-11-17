use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::structure::IrTypeStructRef;
use crate::codegen::ir::ty::{IrType, IrTypeTrait};
use itertools::Itertools;

crate::ir! {
pub struct IrTypeRecord {
    /// Refers to a virtual struct definition.
    pub inner: IrTypeStructRef,
    pub values: Box<[IrType]>,
}
}

impl IrTypeTrait for IrTypeRecord {
    fn visit_children_types<F: FnMut(&IrType) -> bool>(&self, f: &mut F, ir_pack: &IrPack) {
        for ty in self.values.iter() {
            ty.visit_types(f, ir_pack)
        }
    }

    fn safe_ident(&self) -> String {
        self.inner.safe_ident()
    }

    fn rust_api_type(&self) -> String {
        let values = self
            .values
            .iter()
            .map(IrType::rust_api_type)
            .collect_vec()
            .join(",");
        format!("({values},)")
    }
}
