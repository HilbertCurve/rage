use proc_macro::TokenStream;
use quote::quote;
use syn;

pub fn derive_component(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let name = &ast.ident;
    let gen = quote! {
        impl Component for #name {
            fn as_any(&self) -> &dyn std::any::Any {
                self as &dyn std::any::Any
            }
            fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
                self as &mut dyn std::any::Any
            }
            fn type_str() -> &'static str {
                stringify!(#name)
            }
        }
    };
    gen.into()
}

