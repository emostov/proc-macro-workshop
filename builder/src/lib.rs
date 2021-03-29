use proc_macro::{Ident, TokenStream};
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input  as DeriveInput);

	let ident = ast.ident;
	let builder_name = format!("{}Builder", stringify!(name));
	let builder_ident = Ident::new(build_name);

    let expanded = quote! {
		pub struct #builder_ident {
			executable: Option<String>,
			args: Option<Vec<String>>,
			env: Option<Vec<String>>,
			current_dir: Option<String>,
		}

		impl #name {
			pub fn builder() {}
		}
	 };

	expanded.into()
}
