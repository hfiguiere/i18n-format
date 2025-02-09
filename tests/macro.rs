// SPDX-License-Identifier: MIT
// SPDX-FileCopyrightText: (c) 2023 Hubert Figuière

use i18n_format::*;

#[test]
fn i18n_fmt_works() {
    let s;
    let s2;
    i18n_fmt! {
        s = i18n_fmt("This string {}", "formatted");
        s2 = i18n_fmt("This other string {}", "also formatted");
    }
    assert_eq!(&s, "This string formatted");
    assert_eq!(&s2, "This other string also formatted");

    // Testing we can use it twice in the same scope.
    let s3 = i18n_fmt! {
        i18n_fmt("This third string {}", "still formatted")
    };
    assert_eq!(&s3, "This third string still formatted");
}

#[test]
fn i18n_nfmt_works() {
    let s;
    let s2;
    i18n_fmt! {
        s = i18n_nfmt("This string is {}", "These strings are {}", 1, "formatted");
        s2 = i18n_nfmt("This string is {}", "These strings are {}", 2, "formatted");
    }
    assert_eq!(&s, "This string is formatted");
    assert_eq!(&s2, "These strings are formatted");
}

#[test]
fn i18n_mixed_fmt_works() {
    let s;
    let s2;
    i18n_fmt! {
        s = i18n_nfmt("This string is {}", "These strings are {}", 1, "formatted");
        s2 = i18n_fmt("All the strings are {}", "formatted");
    }
    assert_eq!(&s, "This string is formatted");
    assert_eq!(&s2, "All the strings are formatted");
}

#[test]
fn i18n_fmt_error() {
    let s;
    i18n_fmt! {
        // This triggers an error in formatx.
        s = i18n_nfmt("This string is {} {}", "These strings are {} {}", 1, "formatted");
    }
    assert_eq!(&s, "missing placeholders values for: 1 (positional)");
}
