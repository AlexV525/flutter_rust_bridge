use crate::codegen::generator::wire::dart::spec_generator::codec::dco::base::*;
use crate::codegen::generator::wire::dart::spec_generator::codec::dco::decoder::ty::WireDartCodecDcoGeneratorDecoderTrait;
use crate::codegen::ir::func::IrFuncOwnerInfo;
use crate::library::codegen::ir::ty::IrTypeTrait;
use itertools::Itertools;

impl<'a> WireDartCodecDcoGeneratorDecoderTrait for StructRefWireDartCodecDcoGenerator<'a> {
    fn generate_impl_decode_body(&self) -> String {
        let s = self.ir.get(self.context.ir_pack);

        let _has_methods = (self.context.ir_pack.funcs.iter())
            .any(|f| matches!(&f.owner, IrFuncOwnerInfo::Method(_)));

        let inner = s
            .fields
            .iter()
            .enumerate()
            .map(|(idx, field)| {
                format!(
                    "{}: dco_decode_{}(arr[{}]),",
                    field.name.dart_style(),
                    field.ty.safe_ident(),
                    idx
                )
            })
            .collect_vec();

        let inner = inner.join("\n");
        let cast = "final arr = raw as List<dynamic>;".to_string();
        let safe_check = format!("if (arr.length != {}) throw Exception('unexpected arr length: expect {} but see ${{arr.length}}');", s.fields.len(), s.fields.len());
        let dotted_ctor_name = TODO;
        format!(
            "{cast}
                {safe_check}
                return {name}{dotted_ctor_name}({inner});",
            name = s.name.name,
        )
    }
}
