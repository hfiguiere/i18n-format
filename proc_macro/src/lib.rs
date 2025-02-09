// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: (c) 2023 Hubert Figuière

//! Implement the proc macros.
//!

use proc_macro::{Delimiter, Group, TokenStream, TokenTree};
use quote::quote;

#[proc_macro]
/// Wrapper macro to use allow formatting with gettext and ngettext in
/// a way that allow xgettext to find strings for `.po` files as it
/// doesn't support a keyword with a `!`.
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
/// [`i18n-format::i18n_fmt_impl`] or [`i18n-format::i18n_nfmt_impl`],
/// respectively.  Specify `i18n_fmt` and `i18n_nfmt:1,2` as keywords
/// for calls to xgettext.
pub fn i18n_fmt(body: TokenStream) -> TokenStream {
    let mut macro_block: TokenStream = quote!(
        use i18n_format::*;
    )
    .into();
    macro_block.extend(body.into_iter().map(move |tt| {
        if let TokenTree::Ident(ref i) = tt {
            match i.to_string().as_str() {
                "i18n_fmt" => {
                    return TokenTree::Group(Group::new(Delimiter::None, quote!(i18n_fmt_impl!).into()))
                }
                "i18n_nfmt" => {
                    return TokenTree::Group(Group::new(Delimiter::None, quote!(i18n_nfmt_impl!).into()))
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
