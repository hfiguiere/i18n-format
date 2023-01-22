i18n-format
===========

`xgettext` is used to extract strings from source files. Unfortunately
it doesn't support Rust. [The patch has been stuck in
review](https://savannah.gnu.org/bugs/?56774) since 2019.

Fortunately it almost work with Rust. You can specify the keyword
`gettext` or whatever alias you use (I usually use `i18n`), but to use
the formatting version `gettext!` it doesn't work because of the `!`.

This crate provide wrapper macros to use [`gettextrs::gettext!`] and
[`gettextrs::ngettext!`] in a way that allow `xgettext` to find
strings for `.po` files as it doesn't support a keyword with a
`!`. Specify `i18n_fmt` and `i18n_nfmt` as keywords for calls to `xgettext`, and
then write your formatting gettext call like this:

```
use i18n_format::i18n_fmt;

let number = 1;
let s = i18n_fmt! {
    i18n_fmt("This is number {}, make it so !", number)
};
```

is equivalent to
```
use gettextrs::gettext;

let number = 1;
let s = gettext!("This is number {}, make it so !", number);
```

But the string will be extracted.

`i18n_fmt` is just a placeholder in the block for `i18n_fmt!`, and it
will be replaced by a call to `gettext!`. To call `ngettext!` just use
`i18n_nfmt` as a placeholder inside the macro.

## Meson support

The original use of this crate is with gtk-rs apps built with
meson. If you use the regular
[`gtk-rust-template`](https://gitlab.gnome.org/World/Rust/gtk-rust-template/),
internationalization support is mostly setup. The
[`po/meson.build`](https://gitlab.gnome.org/World/Rust/gtk-rust-template/)
file should contain a section like this:

```meson
i18n.gettext(gettext_package, preset: 'glib')
```

Just add to the args the following to the `i18n.gettext` function:
```meson
  args [
    '--keyword=i18n_fmt',
    '--keyword=i18n_nfmt'
  ]
```

If `args` already exist, just add the items to the list.

## License

This crate is licensed under the MIT license.

Author: Hubert Figuière <hub@figuiere.net>
