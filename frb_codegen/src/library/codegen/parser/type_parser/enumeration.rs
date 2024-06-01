use crate::codegen::hir::hierarchical::struct_or_enum::HirEnum;
use crate::codegen::mir::field::{MirField, MirFieldSettings};
use crate::codegen::mir::ident::MirIdent;
use crate::codegen::mir::namespace::{Namespace, NamespacedName};
use crate::codegen::mir::ty::boxed::MirTypeBoxed;
use crate::codegen::mir::ty::delegate::{MirTypeDelegate, MirTypeDelegatePrimitiveEnum};
use crate::codegen::mir::ty::enumeration::{
    MirEnum, MirEnumIdent, MirEnumMode, MirTypeEnumRef, MirVariant, MirVariantKind,
};
use crate::codegen::mir::ty::primitive::MirTypePrimitive;
use crate::codegen::mir::ty::structure::MirStruct;
use crate::codegen::mir::ty::MirType;
use crate::codegen::mir::ty::MirType::{Delegate, EnumRef};
use crate::codegen::parser::attribute_parser::FrbAttributes;
use crate::codegen::parser::type_parser::enum_or_struct::{
    EnumOrStructParser, EnumOrStructParserInfo,
};
use crate::codegen::parser::type_parser::misc::parse_comments;
use crate::codegen::parser::type_parser::structure::structure_compute_default_opaque;
use crate::codegen::parser::type_parser::unencodable::SplayedSegment;
use crate::codegen::parser::type_parser::TypeParserWithContext;
use crate::if_then_some;
use std::collections::HashMap;
use syn::{Attribute, Field, Ident, ItemEnum, Type, Variant, Visibility};

impl<'a, 'b, 'c> TypeParserWithContext<'a, 'b, 'c> {
    pub(crate) fn parse_type_path_data_enum(
        &mut self,
        last_segment: &SplayedSegment,
    ) -> anyhow::Result<Option<MirType>> {
        EnumOrStructParserEnum(self).parse(last_segment, None)
    }

    fn parse_enum(
        &mut self,
        src_enum: &HirEnum,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<MirEnum> {
        let comments = parse_comments(&src_enum.0.src.attrs);
        let raw_variants = src_enum
            .0
            .src
            .variants
            .iter()
            .map(|variant| self.parse_variant(src_enum, variant))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let mode = compute_enum_mode(&raw_variants);
        let variants = maybe_field_wrap_box(raw_variants, mode);

        Ok(MirEnum {
            name,
            wrapper_name,
            comments,
            variants,
            mode,
        })
    }

    fn parse_variant(
        &mut self,
        src_enum: &HirEnum,
        variant: &Variant,
    ) -> anyhow::Result<MirVariant> {
        Ok(MirVariant {
            name: MirIdent::new(variant.ident.to_string()),
            wrapper_name: MirIdent::new(format!("{}_{}", src_enum.0.ident, variant.ident)),
            comments: parse_comments(&variant.attrs),
            kind: match variant.fields.iter().next() {
                None => MirVariantKind::Value,
                Some(Field {
                    attrs,
                    ident: field_ident,
                    ..
                }) => self.parse_variant_kind_struct(src_enum, variant, attrs, field_ident)?,
            },
        })
    }

    fn parse_variant_kind_struct(
        &mut self,
        src_enum: &HirEnum,
        variant: &Variant,
        attrs: &[Attribute],
        field_ident: &Option<Ident>,
    ) -> anyhow::Result<MirVariantKind> {
        let variant_ident = variant.ident.to_string();
        let enum_name = &src_enum.0.namespaced_name;
        let variant_namespace = enum_name.namespace.join(&enum_name.name);
        let attributes = FrbAttributes::parse(attrs)?;
        Ok(MirVariantKind::Struct(MirStruct {
            name: NamespacedName::new(variant_namespace, variant_ident),
            wrapper_name: None,
            is_fields_named: field_ident.is_some(),
            dart_metadata: attributes.dart_metadata(),
            ignore: attributes.ignore(),
            generate_hash: true,
            generate_eq: true,
            comments: parse_comments(attrs),
            fields: variant
                .fields
                .iter()
                .enumerate()
                .map(|(idx, field)| {
                    Ok(MirField {
                        name: MirIdent::new(
                            field
                                .ident
                                .as_ref()
                                .map(ToString::to_string)
                                .unwrap_or_else(|| format!("field{idx}")),
                        ),
                        ty: self.parse_type(&field.ty)?,
                        is_final: true,
                        is_rust_public: Some(matches!(field.vis, Visibility::Public(_))),
                        comments: parse_comments(&field.attrs),
                        default: FrbAttributes::parse(&field.attrs)?.default_value(),
                        settings: MirFieldSettings {
                            is_in_mirrored_enum: src_enum.0.mirror,
                        },
                    })
                })
                .collect::<anyhow::Result<Vec<_>>>()?,
        }))
    }
}

struct EnumOrStructParserEnum<'a, 'b, 'c, 'd>(&'d mut TypeParserWithContext<'a, 'b, 'c>);

impl EnumOrStructParser<MirEnumIdent, MirEnum, HirEnum, ItemEnum>
    for EnumOrStructParserEnum<'_, '_, '_, '_>
{
    fn parse_inner_impl(
        &mut self,
        src_object: &HirEnum,
        name: NamespacedName,
        wrapper_name: Option<String>,
    ) -> anyhow::Result<MirEnum> {
        self.0.parse_enum(src_object, name, wrapper_name)
    }

    fn construct_output(&self, ident: MirEnumIdent) -> anyhow::Result<MirType> {
        let enum_ref = MirTypeEnumRef {
            ident: ident.clone(),
            is_exception: false,
        };
        let enu = self.0.inner.enum_parser_info.object_pool.get(&ident);

        Ok(
            if enu.map(|e| e.mode == MirEnumMode::Complex).unwrap_or(true) {
                EnumRef(enum_ref)
            } else {
                Delegate(MirTypeDelegate::PrimitiveEnum(
                    MirTypeDelegatePrimitiveEnum {
                        ir: enum_ref,
                        // TODO(Desdaemon): Parse #[repr] from enum
                        repr: MirTypePrimitive::I32,
                    },
                ))
            },
        )
    }

    fn src_objects(&self) -> &HashMap<String, &HirEnum> {
        &self.0.inner.src_enums
    }

    fn parser_info(&mut self) -> &mut EnumOrStructParserInfo<MirEnumIdent, MirEnum> {
        &mut self.0.inner.enum_parser_info
    }

    fn dart_code_of_type(&mut self) -> &mut HashMap<String, String> {
        &mut self.0.inner.dart_code_of_type
    }

    fn parse_type_rust_auto_opaque_implicit(
        &mut self,
        namespace: Option<Namespace>,
        ty: &Type,
    ) -> anyhow::Result<MirType> {
        self.0.parse_type_rust_auto_opaque_implicit(namespace, ty)
    }

    fn compute_default_opaque(obj: &MirEnum) -> bool {
        obj.variants
            .iter()
            .filter_map(|variant| if_then_some!(let MirVariantKind::Struct(s) = &variant.kind, s))
            .any(structure_compute_default_opaque)
    }
}

fn maybe_field_wrap_box(mut variants: Vec<MirVariant>, mode: MirEnumMode) -> Vec<MirVariant> {
    if mode == MirEnumMode::Complex {
        for variant in &mut variants {
            if let MirVariantKind::Struct(st) = &mut variant.kind {
                for field in &mut st.fields {
                    ir_type_wrap_box(&mut field.ty);
                }
            }
        }
    }

    variants
}

fn ir_type_wrap_box(ty: &mut MirType) {
    if ty.is_struct_or_enum_or_record() {
        *ty = MirType::Boxed(MirTypeBoxed {
            exist_in_real_api: false,
            inner: Box::new(ty.clone()),
        });
    }
}

fn compute_enum_mode(variants: &[MirVariant]) -> MirEnumMode {
    if variants
        .iter()
        .any(|variant| !matches!(variant.kind, MirVariantKind::Value))
    {
        MirEnumMode::Complex
    } else {
        MirEnumMode::Simple
    }
}
