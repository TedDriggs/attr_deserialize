use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{DeriveInput, Field, Ident, Meta};

use codegen::FromVariantImpl;
use options::{DataShape, OuterFrom, ParseAttribute, ParseData};
use {FromMeta, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FromVariantOptions {
    pub base: OuterFrom,
    pub fields: Option<Ident>,
    pub supports: Option<DataShape>,
}

impl FromVariantOptions {
    pub fn new(di: &DeriveInput) -> Result<Self> {
        (Self {
            base: OuterFrom::start(di),
            fields: Default::default(),
            supports: Default::default(),
        })
        .parse_attributes(&di.attrs)?
        .parse_body(&di.data)
    }
}

impl<'a> From<&'a FromVariantOptions> for FromVariantImpl<'a> {
    fn from(v: &'a FromVariantOptions) -> Self {
        FromVariantImpl {
            base: (&v.base.container).into(),
            ident: v.base.ident.as_ref(),
            fields: v.fields.as_ref(),
            attrs: v.base.attrs.as_ref(),
            attr_names: &v.base.attr_names,
            forward_attrs: v.base.forward_attrs.as_ref(),
            from_ident: v.base.from_ident,
            supports: v.supports.as_ref(),
        }
    }
}

impl ParseAttribute for FromVariantOptions {
    fn parse_nested(&mut self, mi: &Meta) -> Result<()> {
        if mi.path().is_ident("supports") {
            self.supports = FromMeta::from_meta(mi)?;
            Ok(())
        } else {
            self.base.parse_nested(mi)
        }
    }
}

impl ParseData for FromVariantOptions {
    fn parse_field(&mut self, field: &Field) -> Result<()> {
        match field
            .ident
            .as_ref()
            .map(|v| v.to_string())
            .as_ref()
            .map(|v| v.as_str())
        {
            Some("fields") => {
                self.fields = field.ident.clone();
                Ok(())
            }
            _ => self.base.parse_field(field),
        }
    }
}

impl ToTokens for FromVariantOptions {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        FromVariantImpl::from(self).to_tokens(tokens)
    }
}
