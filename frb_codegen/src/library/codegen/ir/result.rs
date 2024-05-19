use crate::codegen::ir::ty::{IrContext, IrType};

crate::ir! {
/// A `Result<T, E>` or a direct type `T`
pub(crate) struct IrMaybeResult {
    pub(crate) normal: IrType,
    pub(crate) error: Option<IrType>,
}
}

impl IrMaybeResult {
    pub(crate) fn visit_types<F: FnMut(&IrType) -> bool>(
        &self,
        f: &mut F,
        ir_context: &impl IrContext,
    ) {
        self.normal.visit_types(f, ir_context);
        if let Some(error) = &self.error {
            error.visit_types(f, ir_context);
        }
    }
}
