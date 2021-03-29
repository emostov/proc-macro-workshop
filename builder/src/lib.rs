use proc_macro2::{Ident, Span};
use quote::{quote};
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(Builder)]
pub fn derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

	let ident = ast.ident;
	let builder_name = format!("{}Builder", ident);
	let builder_ident = Ident::new(&builder_name, Span::call_site());

	let fields = ast.

    let expanded = quote! {
		pub struct #builder_ident {
			executable: Option<String>,
			args: Option<Vec<String>>,
			env: Option<Vec<String>>,
			current_dir: Option<String>,
		}

		impl #ident {
			pub fn builder() -> #builder_ident {
				#builder_ident {
					executable: None,
					args: None,
					env: None,
					current_dir: None,
				}
			}
		}
	 };

	expanded.into()
}
