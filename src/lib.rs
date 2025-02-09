// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: (c) 2025 Hubert Figuière

//! Implementation macro
#![doc = include_str!("../README.md")]

pub use i18n_format_macro::i18n_fmt;
pub use formatx::formatx;
pub use gettextrs::{gettext, ngettext};

#[macro_export]
macro_rules! i18n_fmt_impl {
    ($template:expr) => {
        format!("{}", $crate::gettext($template))
    };
    ($template:expr, $($values:tt)*) => {
        format!("{}", $crate::formatx!($crate::gettext($template), $($values)*).unwrap())
    };
}

#[macro_export]
macro_rules! i18n_nfmt_impl {
    ($templates:expr, $templatep:expr, $count:expr) => {
        format!("{}", $crate::ngettext($templates, $templatep, $count))
    };
    ($templates:expr, $templatep:expr, $count:expr, $($values:tt)*) => {
        format!("{}", $crate::formatx!($crate::ngettext($templates, $templatep, $count), $($values)*).unwrap())
    };
}
