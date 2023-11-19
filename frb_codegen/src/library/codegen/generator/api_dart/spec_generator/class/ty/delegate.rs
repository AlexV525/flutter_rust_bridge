use crate::codegen::generator::api_dart::spec_generator::class::ty::ApiDartGeneratorClassTrait;
use crate::codegen::ir::ty::delegate::{
    IrTypeDelegate, IrTypeDelegateArray, IrTypeDelegatePrimitiveEnum,
};
use crate::library::codegen::generator::api_dart::spec_generator::base::*;
use crate::library::codegen::generator::api_dart::spec_generator::info::ApiDartGeneratorInfoTrait;

impl<'a> ApiDartGeneratorClassTrait for DelegateApiDartGenerator<'a> {
    fn generate_class(&self) -> Option<String> {
        match &self.ir {
            IrTypeDelegate::PrimitiveEnum(IrTypeDelegatePrimitiveEnum { ir, .. }) => {
                EnumRefApiDartGenerator::new(ir.clone(), self.context).generate_class()
            }
            IrTypeDelegate::Array(array) => generate_array(array, self.context),
            _ => None,
        }
    }
}

fn generate_array(array: &IrTypeDelegateArray, context: ApiDartGeneratorContext) -> Option<String> {
    let self_dart_api_type = array.dart_api_type(context);
    let inner_dart_api_type = ApiDartGenerator::new(array.inner(), context).dart_api_type();
    let delegate_dart_api_type =
        ApiDartGenerator::new(array.get_delegate(), context).dart_api_type();

    let array_length = array.length();

    let dart_init_method = match array {
            IrTypeDelegateArray::GeneralArray { .. } => format!(
                "{self_dart_api_type}.init({inner_dart_api_type} fill): super(List<{inner_dart_api_type}>.filled(arraySize,fill));",
            ),
            IrTypeDelegateArray::PrimitiveArray { .. } => format!(
                "{self_dart_api_type}.init(): super({delegate_dart_api_type}(arraySize));",
            ),
        };

    Some(format!(
        "
        class {self_dart_api_type} extends NonGrowableListView<{inner_dart_api_type}> {{
            static const arraySize = {array_length};
            {self_dart_api_type}({delegate_dart_api_type} inner)
                : assert(inner.length == arraySize),
                  super(inner);
            {self_dart_api_type}.unchecked({delegate_dart_api_type} inner)
                : super(inner);
            {dart_init_method}
          }}
        "
    ))
}
