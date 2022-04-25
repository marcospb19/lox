use proc_macro::TokenStream;
use quote::quote;

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

    base.into()
}
