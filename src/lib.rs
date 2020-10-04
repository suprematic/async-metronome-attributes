use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

#[proc_macro_attribute]
pub fn test(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(item as syn::ItemFn);

    let ret = &input.sig.output;
    let name = &input.sig.ident;
    let body = &input.block;
    let attrs = &input.attrs;
    let vis = &input.vis;

    if input.sig.asyncness.is_none() {
        return TokenStream::from(quote_spanned! { input.span() =>
            compile_error!("the function must be declared as 'async'"),
        });
    }

    let result = quote! {
        #[test]
        #(#attrs)*
        #vis fn #name() #ret {
            async_metronome::run(async {
                #body
            });
        }
    };

    result.into()
}
