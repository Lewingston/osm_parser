
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, ItemEnum};

#[proc_macro_derive(FeatureSubType)]
pub fn feature_sub_type_derive(input: TokenStream) -> TokenStream {

    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expand = quote! {
        impl FeatureSubType for #name {

            fn create(&self, attr: &str) -> Option<Feature> {
                match #name::from_str(attr) {
                    Ok(sub_type) => Some(Feature::#name(sub_type)),
                    Err(_) => None
                }
            }

            fn subtype_to_string(&self) -> String {

                self.to_string()
            }
        }
    };

    TokenStream::from(expand)
}


#[proc_macro_attribute]
pub fn feature_sub_type(_attr: TokenStream, item: TokenStream) -> TokenStream {

    let input = parse_macro_input!(item as ItemEnum);

    let attrs    = &input.attrs;
    let vis      = &input.vis;
    let ident    = &input.ident;
    let generics = &input.generics;
    let variants = &input.variants;

    TokenStream::from(quote! {
        #(#attrs)*
        #[derive(
            FeatureSubType,
            Default,
            Display,
            EnumString,
            PartialEq,
            Eq,
            Hash
        )]
        #[strum(serialize_all = "snake_case")]
        #vis enum #ident #generics {
            #[default]
            #variants
        }
    })
}
