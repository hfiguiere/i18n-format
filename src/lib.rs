// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: (c) 2025 Hubert Figuière

#![doc = include_str!("../README.md")]

pub use formatx::formatx;
pub use gettextrs::{gettext, ngettext};
pub use i18n_format_macro::i18n_fmt;

/// Implement i18n_fmt. This is not meant to be called directly
/// but rather by the proc-macro `i18n-fmt`.
///
/// # Panic
///
/// The output code will panic under the same condition as
/// [`formatx::formatx!`]
#[macro_export]
macro_rules! i18n_fmt_impl {
    ($template:expr) => {
        format!("{}", $crate::gettext($template))
    };
    ($template:expr, $($values:tt)*) => {
        format!("{}", $crate::formatx!($crate::gettext($template), $($values)*).unwrap_or_else(|err| err.message()))
    };
}

/// Implement i18n_nfmt. This is not meant to be called directly
/// but rather by the proc-macro `i18n-fmt`.
///
/// # Panic
///
/// The output code will panic under the same condition as
/// [`formatx::formatx!`]
#[macro_export]
macro_rules! i18n_nfmt_impl {
    ($templates:expr, $templatep:expr, $count:expr) => {
        format!("{}", $crate::ngettext($templates, $templatep, $count))
    };
    ($templates:expr, $templatep:expr, $count:expr, $($values:tt)*) => {
        format!("{}", $crate::formatx!($crate::ngettext($templates, $templatep, $count), $($values)*).unwrap_or_else(|err| err.message()))
    };
}
