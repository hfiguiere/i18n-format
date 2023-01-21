// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: (c) 2023 Hubert Figuière

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use quote::quote;

#[proc_macro]
/// Wrapper macro to use [`gettextrs::gettext!`] in a way that allow xgettext
/// to find strings for `.po` files as it doesn't support a keyword with
/// a `!`.
///
/// ```
/// use i18n_format::i18n_fmt;
///
/// let number = 1;
/// let s = i18n_fmt! {
///     i18n_fmt("This is number {}, make it so !", number)
/// };
/// ```
///
/// `i18n_fmt` doesn't exist, but in the block for `i18n_fmt!`
/// it will be replaced by a call to [`gettext!`].
/// Specify `i18n_fmt` as a keyword for calls to xgettext.
pub fn i18n_fmt(body: TokenStream) -> TokenStream {
    i18n_fmt_impl("i18n_fmt", quote!(gettext!).into(), body)
}


fn i18n_fmt_impl(macro_name: &str, out_macro: TokenStream, body: TokenStream) -> TokenStream {
    let mut macro_block: TokenStream = quote!(
        use gettextrs::gettext;
    )
    .into();
    macro_block.extend(body.into_iter().map(move |tt| {
        if let TokenTree::Ident(ref i) = tt {
            if &i.to_string() == macro_name {
                return TokenTree::Group(Group::new(Delimiter::None, out_macro.clone()));
            }
        }
        tt
    }));

    [TokenTree::Group(Group::new(Delimiter::Brace, macro_block))]
        .into_iter()
        .collect()
}
