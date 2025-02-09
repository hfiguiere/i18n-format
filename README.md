i18n-format
===========

`xgettext` is used to extract strings from source files. Unfortunately
it doesn't support Rust. [The patch just got merged in Jan
2025](https://savannah.gnu.org/bugs/?56774). Then it will take a
release and distros to ship it.

Fortunately it almost work with Rust. You can specify the keyword
`gettext` or whatever alias you use (I usually use `i18n`).

The `gettextrs` create doesn't offer a formatting version as the
formatting macro `gettext!` was removed.

These twi crates provide a way to have formatting macros with gettext
and support the extraction, as the macro syntax is not compatible with
xgettext that extract strings for `.po` files. Specify `i18n_fmt` and
`i18n_nfmt` as keywords for calls to `xgettext`, and then write your
formatting gettext call like this:

```
use i18n_format::i18n_fmt;

let number = 1;
let s = i18n_fmt! {
    i18n_fmt("This is number {}, make it so !", number)
};
```

is equivalent to
```ignore
use gettextrs::gettext;

let number = 1;
let s = format!(gettext!("This is number {}, make it so !"), number);
```

The latter wouldn't even compile as `format!` expect a literal.

And the string will be extracted.

`i18n_fmt` is just a placeholder in the block for `i18n_fmt!`, and it
will be replaced by another macro that will call `gettext!`. To call
`ngettext!` just use `i18n_nfmt` as a placeholder inside the macro.

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
    '--keyword=i18n_nfmt:1,2'
  ]
```

If `args` already exist, just add the items to the list.

## Future

This version of the crate has been reworked to work around a breakage
in gettextrs. See [Issue
1](https://github.com/hfiguiere/i18n-format/issues/1)

It turns out that two thing might happen soon:

1. gettext [will have native Rust
   support](https://github.com/gettext-rs/gettext-rs/issues/86#issuecomment-2628889816).
2. gettextrs might offer the formatting macros.

When this happens I hope these crates will be obsolete.

## License

This crate is licensed under the MIT license.

Author: Hubert Figuière <hub@figuiere.net>
