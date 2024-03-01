use proc_macro::TokenStream;

#[proc_macro_derive(Error)]
pub fn derive_error(_: TokenStream) -> TokenStream {}
