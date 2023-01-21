use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use quote::quote;

#[proc_macro]
pub fn i18n_format(body: TokenStream) -> TokenStream {
    return body
        .into_iter()
        .map(|tt| {
            match tt {
                TokenTree::Ident(ref i) => {
                    if &i.to_string() == "i18n_format" {
                        return TokenTree::Group(Group::new(
                            Delimiter::None,
                            quote!(gettext!).into(),
                        ));
                    }
                }
                _ => {}
            }
            tt
        })
        .collect();
}
