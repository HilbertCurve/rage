use proc_macro::{self, TokenStream};
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

pub fn derive(input: TokenStream) -> TokenStream {
    let DeriveInput { ident, .. } = parse_macro_input!(input);
    let output = quote! {
        impl Component for #ident {
            fn as_any(&self) -> &dyn std::any::Any {
                self as &dyn std::any::Any
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self as &mut dyn std::any::Any
            }
            fn type_str() -> &'static str where Self: Sized {
                stringify!(#ident)
            }
        }
    };
    output.into()
}
