i18n_format
===========

`xgettext` is used to extract strings from source files. Unfortunately
it doesn't support Rust. [The patch has been stuck in
review](https://savannah.gnu.org/bugs/?56774) since 2019.

Fortunately it almost work with Rust. You can specify the keyword
`gettext` or whatever alias you use (I usually use `i18n`), but to use
the formatting version `gettext!` it doesn't work because of the `!`.

This crate provide a wrapper macro to use [`gettextrs::gettext!`] in a
way that allow xgettext to find strings for `.po` files as it doesn't
support a keyword with a `!`. Specify `i18n_format` as a keyword for
calls to `xgettext`, and then write your formatting gettext call like
this:

```
use i18n_format::i18n_format;

let number = 1;
let s = i18n_format! {
    i18n_format("This is number {}, make it so !", number)
};
```

is equivalent to
```
use gettextrs::gettext;

let number = 1;
let s = gettext!("This is number {}, make it so !", number);
```

But the string will be extracted.

`i18n_format` doesn't exist, but in the block for `i18n_format!`
it will be replaced by a call to `gettext!`.

