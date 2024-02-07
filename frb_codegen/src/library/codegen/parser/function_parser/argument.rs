use crate::codegen::ir::field::{IrField, IrFieldSettings};
use crate::codegen::ir::func::{IrFuncMode, IrFuncOwnerInfo};
use crate::codegen::ir::ident::IrIdent;
use crate::codegen::ir::ty::boxed::IrTypeBoxed;
use crate::codegen::ir::ty::IrType;
use crate::codegen::ir::ty::IrType::Boxed;
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::function_parser::{
    FunctionParser, FunctionPartialInfo, STREAM_SINK_IDENT,
};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::TypeParserParsingContext;
use crate::if_then_some;
use anyhow::{bail, ensure, Context};
use syn::*;

impl<'a, 'b> FunctionParser<'a, 'b> {
    pub(super) fn parse_fn_arg(
        &mut self,
        argument_index: usize,
        sig_input: &FnArg,
        owner: &IrFuncOwnerInfo,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<FunctionPartialInfo> {
        match sig_input {
            FnArg::Typed(ref pat_type) => {
                self.parse_fn_arg_typed(argument_index, context, pat_type)
            }
            FnArg::Receiver(ref receiver) => self.parse_fn_arg_receiver(owner, context, receiver),
        }
    }

    fn parse_fn_arg_typed(
        &mut self,
        argument_index: usize,
        context: &TypeParserParsingContext,
        pat_type: &PatType,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let ty = pat_type.ty.as_ref();

        if let Type::Path(TypePath { path, .. }) = &ty {
            if let Some(ans) = self.parse_fn_arg_type_stream_sink(path, argument_index, context)? {
                return Ok(ans);
            }
        }

        partial_info_for_normal_type(self.type_parser.parse_type(ty, context)?, pat_type)
    }

    fn parse_fn_arg_receiver(
        &mut self,
        owner: &IrFuncOwnerInfo,
        context: &TypeParserParsingContext,
        receiver: &Receiver,
    ) -> anyhow::Result<FunctionPartialInfo> {
        let method = if_then_some!(let IrFuncOwnerInfo::Method(method) = owner, method)
            .context("`self` must happen within methods")?;

        let ty_raw =
            self.parse_fn_arg_receiver_attempt(&method.enum_or_struct_name.name, context)?;
        let ty = match ty_raw {
            IrType::RustAutoOpaque(ty_raw) => self.type_parser.transform_rust_auto_opaque(
                &ty_raw,
                |raw| generate_ref_type_considering_reference(raw, receiver),
                context,
            )?,
            _ => ty_raw,
        };

        let name = "that".to_owned();

        if let IrType::StructRef(s) = &ty {
            if s.get(self.type_parser).ignore {
                return Ok(FunctionPartialInfo {
                    ignore_func: true,
                    ..Default::default()
                });
            }

            ensure!(TODO, "TODO");
        }

        partial_info_for_normal_type_raw(ty, &receiver.attrs, name)
    }

    fn parse_fn_arg_receiver_attempt(
        &mut self,
        ty: &str,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<IrType> {
        let ty: Type = parse_str(ty)?;
        self.type_parser.parse_type(&ty, context)
    }

    fn parse_fn_arg_type_stream_sink(
        &mut self,
        path: &Path,
        argument_index: usize,
        context: &TypeParserParsingContext,
    ) -> anyhow::Result<Option<FunctionPartialInfo>> {
        let last_segment = path.segments.last().unwrap();
        Ok(if last_segment.ident == STREAM_SINK_IDENT {
            match &last_segment.arguments {
                PathArguments::AngleBracketed(AngleBracketedGenericArguments { args, .. })
                    if !args.is_empty() =>
                {
                    // Unwrap is safe here because args.len() >= 1
                    match args.first().unwrap() {
                        GenericArgument::Type(t) => Some(partial_info_for_stream_sink_type(
                            self.type_parser.parse_type(t, context)?,
                            argument_index,
                        )?),
                        _ => None,
                    }
                }
                _ => None,
            }
        } else {
            None
        })
    }
}

fn partial_info_for_normal_type(
    ty_raw: IrType,
    pat_type: &PatType,
) -> anyhow::Result<FunctionPartialInfo> {
    let name = parse_name_from_pat_type(pat_type)?;
    partial_info_for_normal_type_raw(ty_raw, &pat_type.attrs, name)
}

fn partial_info_for_normal_type_raw(
    ty_raw: IrType,
    attrs: &[Attribute],
    name: String,
) -> anyhow::Result<FunctionPartialInfo> {
    let attributes = FrbAttributes::parse(attrs)?;
    let ty = auto_add_boxed(ty_raw);
    Ok(FunctionPartialInfo {
        inputs: vec![IrField {
            name: IrIdent::new(name),
            ty,
            is_final: true,
            comments: parse_comments(attrs),
            default: attributes.default_value(),
            settings: IrFieldSettings::default(),
        }],
        ..Default::default()
    })
}

fn auto_add_boxed(ty: IrType) -> IrType {
    if ty.is_struct_or_enum_or_record() {
        Boxed(IrTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(ty),
        })
    } else {
        ty
    }
}

fn partial_info_for_stream_sink_type(
    ty: IrType,
    argument_index: usize,
) -> anyhow::Result<FunctionPartialInfo> {
    Ok(FunctionPartialInfo {
        ok_output: Some(ty),
        mode: Some(IrFuncMode::Stream { argument_index }),
        ..Default::default()
    })
}

fn parse_name_from_pat_type(pat_type: &PatType) -> anyhow::Result<String> {
    if let Pat::Ident(ref pat_ident) = *pat_type.pat {
        Ok(format!("{}", pat_ident.ident))
    } else {
        // This branch simply halts the generator with an error message, so we do not test it
        // frb-coverage:ignore-start
        bail!(
            "Unexpected pattern: {}",
            quote::quote!(#pat_type).to_string(),
        )
        // frb-coverage:ignore-end
    }
}

fn generate_ref_type_considering_reference(raw: &str, receiver: &Receiver) -> String {
    if receiver.reference.is_some() {
        if receiver.mutability.is_some() {
            format!("&mut {raw}")
        } else {
            format!("&{raw}")
        }
    } else {
        raw.to_owned()
    }
}
