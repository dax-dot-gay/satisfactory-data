mod model;
use manyhow::manyhow;
use proc_macro2::TokenStream;

#[manyhow]
#[proc_macro_attribute]
pub fn model(args: TokenStream, input: TokenStream) -> manyhow::Result<TokenStream> {
    model::model_impl(args, input)
}
