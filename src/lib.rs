// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: (c) 2023 Hubert Figuière

//! Implement the proc macros.
//!
#![doc = include_str!("../README.md")]

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
/// it will be replaced by a call to [`gettextrs::gettext!`].
/// Specify `i18n_fmt` as a keyword for calls to xgettext.
pub fn i18n_fmt(body: TokenStream) -> TokenStream {
    i18n_fmt_impl("i18n_fmt", quote!(gettext!).into(), body)
}

#[proc_macro]
/// Wrapper macro to use [`gettextrs::ngettext!`] in a way that allow xgettext
/// to find strings for `.po` files as it doesn't support a keyword with
/// a `!`. It is like [`i18n_fmt!`] but for calling `ngettext` in order to support
/// plural forms.
///
/// Refer to [`i18n_fmt!`] for usage, with the arguments expected being the same as
/// for [`gettextrs::ngettext!`].
pub fn i18n_nfmt(body: TokenStream) -> TokenStream {
    i18n_fmt_impl("i18n_nfmt", quote!(ngettext!).into(), body)
}


/// The implementation for the macro, with `macro_name` and `out_macro` as
/// parameters.
fn i18n_fmt_impl(macro_name: &str, out_macro: TokenStream, body: TokenStream) -> TokenStream {
    let mut macro_block: TokenStream = quote!(
        use gettextrs::*;
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
