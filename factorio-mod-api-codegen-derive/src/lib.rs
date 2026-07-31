use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{Data, DeriveInput, Fields, parse_macro_input};

#[proc_macro_derive(Visitable)]
pub fn derive_visitable(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let impl_body = match &input.data {
        Data::Struct(s) => derive_struct(name, &s.fields),
        Data::Enum(e) => derive_enum(name, e),
        Data::Union(_) => panic!("Visitable cannot be derived for unions"),
    };

    TokenStream::from(quote! {
        impl ::factorio_mod_api_codegen::visitor::Visitable for #name {
            #impl_body
        }
    })
}

fn derive_struct(_name: &syn::Ident, fields: &Fields) -> TokenStream2 {
    match fields {
        // Newtype struct: struct Foo(pub T) — leaf only, no visit_node
        Fields::Unnamed(f) if f.unnamed.len() == 1 => {
            quote! {
                fn visit_leaf(&self, visitor: &mut dyn ::factorio_mod_api_codegen::visitor::Visitor) -> bool {
                    visitor.grab(self)
                }
            }
        }
        // Named fields struct
        Fields::Named(f) => {
            let visits: Vec<TokenStream2> = f.named.iter().map(|field| {
                let ident = field.ident.as_ref().unwrap();
                let name_str = ident.to_string();
                // strip r# prefix for the path string
                let name_str = name_str.trim_start_matches("r#");
                quote! {
                    visitor.visit(&self.#ident, #name_str.to_string());
                }
            }).collect();

            quote! {
                fn visit_leaf(&self, visitor: &mut dyn ::factorio_mod_api_codegen::visitor::Visitor) -> bool {
                    visitor.grab(self)
                }
                fn visit_node(&self, visitor: &mut dyn ::factorio_mod_api_codegen::visitor::Visitor) {
                    #(#visits)*
                }
            }
        }
        // Unit or multi-field tuple struct — just leaf
        _ => {
            quote! {
                fn visit_leaf(&self, visitor: &mut dyn ::factorio_mod_api_codegen::visitor::Visitor) -> bool {
                    visitor.grab(self)
                }
            }
        }
    }
}

fn derive_enum(name: &syn::Ident, data: &syn::DataEnum) -> TokenStream2 {
    let arms: Vec<TokenStream2> = data.variants.iter().map(|v| {
        let vname = &v.ident;
        let vname_str = vname.to_string();
        match &v.fields {
            Fields::Unnamed(f) if f.unnamed.len() == 1 => quote! {
                #name::#vname(v) => visitor.visit(v, #vname_str.to_string()),
            },
            Fields::Unnamed(_) => quote! {
                #name::#vname(..) => {},
            },
            Fields::Named(_) => quote! {
                #name::#vname { .. } => {},
            },
            Fields::Unit => quote! {
                #name::#vname => {},
            },
        }
    }).collect();

    quote! {
        fn visit_leaf(&self, visitor: &mut dyn ::factorio_mod_api_codegen::visitor::Visitor) -> bool {
            visitor.grab(self)
        }
        fn visit_node(&self, visitor: &mut dyn ::factorio_mod_api_codegen::visitor::Visitor) {
            match self {
                #(#arms)*
            }
        }
    }
}
