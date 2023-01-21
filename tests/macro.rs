use i18n_format::*;

use gettextrs::gettext;

#[test]
fn i18n_format_works() {
    let s;
    i18n_format! {
        s = i18n_format("This string {}", "formatted");
    }
    assert_eq!(&s, "This string formatted");
}
