use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{ItemEnum, ItemStruct, Path, Token, parse::Parser, punctuated::Punctuated};

pub fn model_impl(args: TokenStream, input: TokenStream) -> manyhow::Result<TokenStream> {
    if syn::parse2::<ItemStruct>(input.clone()).is_err()
        && syn::parse2::<ItemEnum>(input.clone()).is_err()
    {
        Err(syn::Error::new(
            Span::call_site(),
            "This macro only supports structs and enums.",
        ))?;
    }
    let mut args = Punctuated::<Path, Token![,]>::parse_terminated.parse2(args)?;
    args.push(syn::parse_str::<Path>("serde::Serialize")?);
    args.push(syn::parse_str::<Path>("serde::Deserialize")?);
    args.push(syn::parse_str::<Path>("Clone")?);
    args.push(syn::parse_str::<Path>("Debug")?);

    Ok(quote! {
        #[cfg_attr(feature = "specta-1", derive(specta_01::Type))]
        #[cfg_attr(feature = "specta-1", specta(crate = specta_01))]
        #[cfg_attr(feature = "specta-2", derive(specta_02::Type))]
        #[cfg_attr(feature = "specta-2", specta(crate = specta_02))]
        #[derive(#args)]
        #input
    })
}
