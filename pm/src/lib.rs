use proc_macro::TokenStream;
use quote::quote;
use proc_macro2::{Ident, Span};
use proc_macro2::TokenStream as TokenStream2;

struct ExpressionStruct<'a> {
    name: &'a str,
    fields: &'a [(&'a str, &'a str)],
}

const DATA: &[ExpressionStruct] = &[
    ExpressionStruct {
        name: "BinaryExpression",
        fields: &[
            ("left", "ExpressionBox"),
            ("operator", "Token"),
            ("right", "ExpressionBox"),
        ],
    },
    ExpressionStruct {
        name: "GroupingExpression",
        fields: &[
            ("expression", "ExpressionBox"),
        ],
    },
    ExpressionStruct {
        name: "LiteralExpression",
        fields: &[
            ("value", "Token"),
        ],
    },
    ExpressionStruct {
        name: "UnaryExpression",
        fields: &[
            ("operator", "Token"),
            ("expression", "ExpressionBox"),
        ],
    },
];

#[proc_macro]
pub fn make_expressions(_item: TokenStream) -> TokenStream {
    let base = quote! {
        // #![cfg_attr(not(test), allow(unused))]
        use std::fmt;

        use crate::token::Token;

        pub trait Expression: fmt::Display {}

        impl Expression for BinaryExpression {}
        impl Expression for GroupingExpression {}
        impl Expression for LiteralExpression {}
        impl Expression for UnaryExpression {}

        pub type ExpressionBox = Box<dyn Expression>;
    };
    
    let structs = DATA
        .into_iter()
        .map(|expr_struct| define_expr(expr_struct))
        .collect::<TokenStream2>();

    quote! {
        #base

        #structs
    }.into()
}

fn define_expr(expr: &ExpressionStruct) -> TokenStream2 {
    let ExpressionStruct { name, fields } = expr;

    let name = Ident::new(name, Span::call_site());

    let fields = fields.into_iter()
        .map(|(f, ty)| {
            (Ident::new(f, Span::call_site()), Ident::new(ty, Span::call_site()))
        }).collect::<Vec<(Ident, Ident)>>();

    let pub_typed_fields = fields
        .iter()
        .map(|(f, ty)| quote!{ pub #f: #ty });
    let typed_fields = fields
        .iter()
        .map(|(f, ty)| quote!{ #f: #ty });
    let untyped_fields = fields
        .iter()
        .map(|(f, _ty)| quote!{ #f });

    quote!{
        pub struct #name {
            #(#pub_typed_fields),*
        }

        impl #name {
            pub fn new(#(#typed_fields),*) -> Self {
                Self { #(#untyped_fields),* }
            }
        }
    }
}
