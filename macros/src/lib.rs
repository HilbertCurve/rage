mod component;
mod error;

use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn component_derive(input: TokenStream) -> TokenStream {
    component::derive(input)
}

#[proc_macro_derive(Error)]
pub fn error_derive(input: TokenStream) -> TokenStream {
    error::derive(input)
}
