mod component_derive;

use proc_macro::TokenStream;

#[proc_macro_derive(Component)]
pub fn derive_component(input: TokenStream) -> TokenStream {
    component_derive::derive_component(input)
}

