use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::dart_opaque::IrTypeDartOpaque;
use crate::codegen::ir::ty::delegate::{IrTypeDelegate, IrTypeDelegateTime};
use crate::codegen::ir::ty::dynamic::IrTypeDynamic;
use crate::codegen::ir::ty::primitive::IrTypePrimitive;
use crate::codegen::ir::ty::primitive_list::IrTypePrimitiveList;
use crate::codegen::ir::ty::rust_opaque::IrTypeRustOpaque;
use crate::codegen::ir::ty::unencodable::IrTypeUnencodable;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::{
    Boxed, DartOpaque, Delegate, Dynamic, Primitive, PrimitiveList, RustOpaque, Unencodable,
};
use crate::codegen::parser::type_parser::unencodable::ArgsRefs::Generic;
use crate::codegen::parser::type_parser::unencodable::{splay_segments, SplayedSegment};
use crate::codegen::parser::type_parser::TypeParserWithContext;
use anyhow::bail;

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_concrete(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<IrType>> {
        Ok(Some(match last_segment {
            ("Duration", None) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Duration)),
            ("NaiveDateTime", None) => Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Naive)),
            ("DateTime", Some(Generic(args))) => parse_datetime(args)?,

            ("Uuid", None) => Delegate(IrTypeDelegate::Uuid),
            ("String", None) => Delegate(IrTypeDelegate::String),
            ("Backtrace", None) => Delegate(IrTypeDelegate::Backtrace),

            ("DartAbi", None) => Dynamic(IrTypeDynamic),

            ("DartOpaque", None) => DartOpaque(IrTypeDartOpaque {}),

            ("RustOpaque", Some(Generic([Delegate(IrTypeDelegate::Array(array_delegate))]))) => {
                Delegate(IrTypeDelegate::Array(array_delegate.clone()))
            }

            ("RustOpaque", Some(Generic([Primitive(IrTypePrimitive::Unit)]))) => RustOpaque(
                IrTypeRustOpaque::new(TODO, Primitive(IrTypePrimitive::Unit)),
            ),

            ("RustOpaque", Some(Generic([ty]))) => {
                RustOpaque(IrTypeRustOpaque::new(TODO, ty.clone()))
            }

            (
                "ZeroCopyBuffer",
                Some(Generic([PrimitiveList(IrTypePrimitiveList { primitive })])),
            ) => Delegate(IrTypeDelegate::ZeroCopyBufferVecPrimitive(
                primitive.clone(),
            )),

            ("Box", Some(Generic([inner]))) => Boxed(IrTypeBoxed {
                exist_in_real_api: true,
                inner: Box::new(inner.clone()),
            }),

            _ => return Ok(None),
        }))
    }
}

fn parse_datetime(args: &[IrType]) -> anyhow::Result<IrType> {
    if let [Unencodable(IrTypeUnencodable { segments, .. })] = args {
        return Ok(match &splay_segments(segments)[..] {
            [("DateTime", None), ("Utc", None)] => {
                Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Utc))
            }

            [("DateTime", None), ("Local", None)] => {
                Delegate(IrTypeDelegate::Time(IrTypeDelegateTime::Local))
            }

            _ => bail!("Invalid DateTime generic"),
        });
    }
    bail!("Invalid DateTime generic")
}
