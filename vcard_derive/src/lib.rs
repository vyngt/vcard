extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

#[proc_macro_derive(VCard)]
pub fn derive_vcard_body(input: TokenStream) -> TokenStream {
    let mut insert_tokens = vec![];
    let mut new_tokens = vec![];

    let parsed_input: DeriveInput = parse_macro_input!(input);
    let struct_name = parsed_input.ident;
    match parsed_input.data {
        Data::Struct(s) => {
            if let Fields::Named(name_fields) = s.fields {
                let arr_name = name_fields.named;
                insert_tokens.push(quote! {
                    content.push_str("VERSION:");
                    content.push_str(Self::version());
                    content.push_str("\n");
                });
                for n in arr_name {
                    let field = n.ident.unwrap();

                    let insert_token = quote! {
                        content.push_str(&self.#field.to_content());
                    };
                    insert_tokens.push(insert_token);

                    let f_type = n.ty;

                    let new_token = quote! {
                        #field: #f_type::new(),
                    };
                    new_tokens.push(new_token);
                }
            }
        }
        Data::Enum(_s) => panic!("Error type: DataEnum"),
        Data::Union(_s) => panic!("Error type: DataUnion"),
    }

    let tokens = quote! {
        impl #struct_name {
            pub fn new() -> Self {
                Self {
                    #(#new_tokens)*
                }
            }

            fn generate_vcard_content(&self) -> String {
                let mut content = String::from("");
                #(#insert_tokens)*
                content
            }

            pub fn generate_vcard(&self) -> String {
                format!("BEGIN:VCARD\n{}END:VCARD", self.generate_vcard_content().as_str())
            }
        }

    };

    proc_macro::TokenStream::from(tokens)
}
