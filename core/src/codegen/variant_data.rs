use proc_macro2::TokenStream;

use ast::Fields;
use ast::Style;
use codegen::Field;

pub struct FieldsGen<'a>(pub &'a Fields<Field<'a>>);

impl<'a> FieldsGen<'a> {
    pub(in codegen) fn declarations(&self) -> TokenStream {
        match *self.0 {
            Fields {
                style: Style::Struct,
                ref fields,
            } => {
                let vdr = fields.iter().map(Field::as_declaration);
                quote!(#(#vdr)*)
            }
            _ => panic!("FieldsGen doesn't support tuples yet"),
        }
    }

    /// Generate the loop which walks meta items looking for property matches.
    pub(in codegen) fn core_loop(&self) -> TokenStream {
        let arms = self.0.as_ref().map(Field::as_match);

        quote!(
            for __item in __items {
                if let ::syn::NestedMeta::Meta(ref __inner) = *__item {
                    let __name = __inner.name().to_string();
                    match __name.as_str() {
                        #(#arms)*
                        __other => {
                            __errors.push(::darling::Error::unknown_field(__other).with_span(__inner));
                        }
                    }
                }
            }
        )
    }

    pub fn require_fields(&self) -> TokenStream {
        match *self.0 {
            Fields {
                style: Style::Struct,
                ref fields,
            } => {
                let checks = fields.iter().map(Field::as_presence_check);
                quote!(#(#checks)*)
            }
            _ => panic!("FieldsGen doesn't support tuples for requirement checks"),
        }
    }

    pub(in codegen) fn initializers(&self) -> TokenStream {
        let inits = self.0.as_ref().map(Field::as_initializer);

        quote!(#(#inits),*)
    }
}
