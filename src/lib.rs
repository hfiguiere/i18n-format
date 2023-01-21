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
    let mut macro_block: TokenStream = quote!(
        use gettextrs::gettext;
    )
    .into();
    macro_block.extend(body.into_iter().map(|tt| {
        match tt {
            TokenTree::Ident(ref i) => {
                if &i.to_string() == "i18n_fmt" {
                    return TokenTree::Group(Group::new(Delimiter::None, quote!(gettext!).into()));
                }
            }
            _ => {}
        }
        tt
    }));

    [TokenTree::Group(Group::new(Delimiter::Brace, macro_block))]
        .into_iter()
        .collect()
}
