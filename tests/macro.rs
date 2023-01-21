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
