// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: (c) 2025 Hubert Figuière

#![doc = include_str!("../README.md")]

pub use formatx::formatx;
pub use gettextrs::{gettext, ngettext};
#[cfg(feature = "legacy")]
pub use i18n_format_macro::i18n_fmt;

/// Format a string with internationalization of the format string.
/// Use the same formating rules as [`formatx::formatx!`].
///
/// This is the implementation when using the proc-macro `i18n-fmt`
/// with feature legacy.
///
/// # Panic
///
/// The output code will panic under the same condition as
/// [`formatx::formatx!`]
#[macro_export]
macro_rules! i18n_format {
    ($template:expr) => {
        format!("{}", $crate::gettext($template))
    };
    ($template:expr, $($values:tt)*) => {
        format!("{}", $crate::formatx!($crate::gettext($template), $($values)*).unwrap_or_else(|err| err.message()))
    };
}

/// Format a string with internationalization of the format string,
/// with plurals. Use the same formating rules as
/// [`formatx::formatx!`].
///
/// This is the implementation when using the proc-macro `i18n-fmt`
/// with feature legacy.
///
/// # Panic
///
/// The output code will panic under the same condition as
/// [`formatx::formatx!`]
#[macro_export]
macro_rules! i18n_nformat {
    ($templates:expr, $templatep:expr, $count:expr) => {
        format!("{}", $crate::ngettext($templates, $templatep, $count))
    };
    ($templates:expr, $templatep:expr, $count:expr, $($values:tt)*) => {
        format!("{}", $crate::formatx!($crate::ngettext($templates, $templatep, $count), $($values)*).unwrap_or_else(|err| err.message()))
    };
}
