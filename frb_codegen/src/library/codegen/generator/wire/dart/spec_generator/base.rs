use crate::codegen::generator::acc::Acc;
use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::misc::target::Target;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::dart::spec_generator::api2wire::ty::WireDartGeneratorApi2wireTrait;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::generator::wire::rust::spec_generator::base::WireRustGeneratorContext;
use crate::codegen::ir::pack::IrPack;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::IrTypeDelegate;
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::enumeration::IrTypeEnumRef;
use crate::codegen::ir::ty::general_list::IrTypeGeneralList;
use crate::codegen::ir::ty::optional::IrTypeOptional;
use crate::codegen::ir::ty::optional_list::IrTypeOptionalList;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::record::IrTypeRecord;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaqueRef;
use crate::codegen::ir::ty::structure::IrTypeStructRef;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::*;
use crate::codegen_generator_structs;
use crate::library::codegen::generator::api_dart::spec_generator::base::ApiDartGeneratorContext;
use enum_dispatch::enum_dispatch;
use paste::paste;

#[enum_dispatch(WireDartGeneratorImplTrait)]
#[enum_dispatch(WireDartGeneratorMiscTrait)]
#[enum_dispatch(WireDartGeneratorApi2wireTrait)]
#[enum_dispatch(WireDartGeneratorWire2apiTrait)]
pub(crate) enum WireDartGenerator<'a> {
    Boxed(BoxedWireDartGenerator<'a>),
    DartOpaque(DartOpaqueWireDartGenerator<'a>),
    Delegate(DelegateWireDartGenerator<'a>),
    Dynamic(DynamicWireDartGenerator<'a>),
    EnumRef(EnumRefWireDartGenerator<'a>),
    GeneralList(GeneralListWireDartGenerator<'a>),
    Optional(OptionalWireDartGenerator<'a>),
    OptionalList(OptionalListWireDartGenerator<'a>),
    Primitive(PrimitiveWireDartGenerator<'a>),
    PrimitiveList(PrimitiveListWireDartGenerator<'a>),
    Record(RecordWireDartGenerator<'a>),
    RustOpaqueRef(RustOpaqueRefWireDartGenerator<'a>),
    StructRef(StructRefWireDartGenerator<'a>),
    Unencodable(UnencodableWireDartGenerator<'a>),
}

codegen_generator_structs!(
    WireDartGenerator;

    Boxed,
    DartOpaque,
    Delegate,
    Dynamic,
    EnumRef,
    GeneralList,
    Optional,
    OptionalList,
    Primitive,
    PrimitiveList,
    Record,
    RustOpaqueRef,
    StructRef,
    Unencodable,
);

#[derive(Debug, Clone, Copy)]
pub(crate) struct WireDartGeneratorContext<'a> {
    pub(crate) ir_pack: &'a IrPack,
    pub(crate) config: &'a GeneratorWireDartInternalConfig,
    pub(crate) wire_rust_config: &'a GeneratorWireRustInternalConfig,
    pub(crate) api_dart_config: &'a GeneratorApiDartInternalConfig,
}

impl WireDartGeneratorContext<'_> {
    pub(crate) fn as_wire_rust_context(&self) -> WireRustGeneratorContext {
        WireRustGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.wire_rust_config,
            wire_dart_config: self.config,
            api_dart_config: self.api_dart_config,
        }
    }

    pub(crate) fn as_api_dart_context(&self) -> ApiDartGeneratorContext {
        ApiDartGeneratorContext {
            ir_pack: self.ir_pack,
            config: self.api_dart_config,
        }
    }
}
