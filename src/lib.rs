// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: (c) 2023 Hubert Figuière

//! Implement the proc macros.
//!
#![doc = include_str!("../README.md")]

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use quote::quote;

#[proc_macro]
/// Wrapper macro to use [`gettextrs::gettext!`] or
/// [`gettextrs::ngettext!`] in a way that allow xgettext to find
/// strings for `.po` files as it doesn't support a keyword with a
/// `!`.
///
/// ```
/// use i18n_format::i18n_fmt;
///
/// let number = 1;
/// let s = i18n_fmt! {
///     i18n_fmt("This is number {}, make it so !", number)
/// };
///
/// let count = 2;
/// let message = i18n_fmt! {
///     i18n_nfmt("Counted {} item", "Counted {} items", count, count)
/// };
/// ```
///
/// Both `i18n_fmt` and `i18n_nfmt` are placeholders, in the block for
/// `i18n_fmt!` either will be replaced by a call to
/// [`gettextrs::gettext!`] or [`gettextrs::ngettext!`], respectively.
/// Specify `i18n_fmt` and `i18n_nfmt` as keywords for calls to
/// xgettext.
pub fn i18n_fmt(body: TokenStream) -> TokenStream {
    let mut macro_block: TokenStream = quote!(
        use gettextrs::*;
    )
    .into();
    macro_block.extend(body.into_iter().map(move |tt| {
        if let TokenTree::Ident(ref i) = tt {
            match i.to_string().as_str() {
                "i18n_fmt" => {
                    return TokenTree::Group(Group::new(Delimiter::None, quote!(gettext!).into()))
                }
                "i18n_nfmt" => {
                    return TokenTree::Group(Group::new(Delimiter::None, quote!(ngettext!).into()))
                }
                _ => {}
            }
        }
        tt
    }));

    [TokenTree::Group(Group::new(Delimiter::Brace, macro_block))]
        .into_iter()
        .collect()
}
