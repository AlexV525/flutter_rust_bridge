use crate::{ir::IrTypeDynamic, target::Acc, type_dart_generator_struct};

use super::{TypeDartGeneratorTrait, TypeGeneratorContext};

type_dart_generator_struct!(TypeDynamicGenerator, IrTypeDynamic);

impl TypeDartGeneratorTrait for TypeDynamicGenerator<'_> {
    fn api2wire_body(
        &self,
        _shared_dart_api2wire_funcs: &Option<Acc<String>>,
    ) -> Acc<Option<String>> {
        Acc::default()
    }
    fn wire2api_body(&self) -> String {
        "return raw;".into()
    }
}
