i18n-format
===========

**Note: This crate has been updated from 0.3.0 in how it works, read
carefully**

**See also the update section further down to port**

This crate is a helper for using gettext. `format!` doesn't support
non static formatting strings. The `gettextrs` crate doesn't offer a
formatting version as the formatting macro `gettext!` was
removed. Using alternatives with gettext lead to relatively verbose
code.

To use the crate just add the following to your Cargo.toml

```toml
i18n-format = "0.4.0"
```

With this crate, getting a formatted localised string is as simple as
this:

```
use i18n_format::{i18n_format, i18n_nformat};

let filename = "file name";
let message = i18n_format!("Loading \"{}\"...", filename);

let count = 2;
let message = i18n_nformat!("Found {} file.",
                            "Found {} files.", count, count);
```

Then with the proper keyword, `xgettext` (starting with 0.24) will
properly extract the strings. See meson support for details on setting
the keywords.

The goal over time is to make this crate obsolete, but in the mean
time it is here to help.

If you can't guarantee `xgettext` 0.25 as part of the developer setup,
you can use the legacy mode.

## Legacy mode

The legacy mode is enabled with the `legacy` crate feature.

```toml
i18n-format = { version = "0.4.0", features = "legacy" }
```

`xgettext` is used to extract strings from source files. Unfortunately
it doesn't support Rust. [The patch got merged in Jan
2025](https://savannah.gnu.org/bugs/?56774). This has been released
for 0.24.

Fortunately it almost work. You can specify the keyword `gettext` or
whatever alias you use (I usually use `i18n`). But it can't take a
"keyword" that ends in `!` like a Rust macro which make using
localised string format harder to use.

The legacy mode use a proc macro to enclose a section of localised
string use with a placholder function call that is compatible with
xgettext extraction for `.po` files, and a compile time replacement to
produce a macro for formatting. Specify `i18n_fmt` and `i18n_nfmt`
(the two placholders) as keywords for calls to `xgettext`, and then
write your formatting gettext call like this:

```ignore
use i18n_format::i18n_fmt;

let number = 1;
let s = i18n_fmt! {
    i18n_fmt("This is number {}, make it so !", number)
};
```

is equivalent to

```
use formatx::formatx;
use gettextrs::gettext;

let number = 1;
let s = formatx!(gettext("This is number {}, make it so !"), number);
```

We use the `formatx!` macro from the crate of the same name. The
latter example wouldn't even compile if we used `format!`, as it
expects a literal. And the strings will be extracted with `xgettext`.

The `i18n_fmt` function is just a placeholder in the inner block of q
`i18n_fmt!`, and it will be replaced by another macro that will call
`gettext`. To call `ngettext` for plural forms, just use `i18n_nfmt`
as a placeholder inside the macro.

## Error

The underlying [`formatx::formatx!`] macro will panic in some
cases. When it returns an error, `i18n_format!` will output the error
instead of the string.

## Meson support

The original use case of this crate was for gtk-rs apps built with
meson. If you use the regular
[`gtk-rust-template`](https://gitlab.gnome.org/World/Rust/gtk-rust-template/),
internationalisation support is mostly setup. The
[`po/meson.build`](https://gitlab.gnome.org/World/Rust/gtk-rust-template/)
file should contain a section like this:

```meson
i18n.gettext(gettext_package, preset: 'glib')
```

Just add to the args the following to the `i18n.gettext` function:
```meson
  args [
    '--keyword=i18n_format!',
    '--keyword=i18n_nformat!:1,2'
  ]
```

This only work of you have xgettext 0.24 or older.

If you use the legacy form, for older gettext, do this:

```meson
  args [
    '--keyword=i18n_fmt',
    '--keyword=i18n_nfmt:1,2'
  ]
```

If `args` already exist, just add the items to the list.

Both can be commbined without side effect if your codebase hasn't been
migrated.

## Upgrading from 0.3.0

Upgrading from 0.3.0, just build with the `legacy` feature and the old
syntax will work. If you have gettext 0.24 or later, you can starting
using the new form along with the legacy one. Don't forget to update
the keywords xgettext if you also use the new form. If you migrate to
the new simpler syntax, just keep the default features for the crate
and update the keywords for xgettext.

## Future

Now that native Rust support exist in gettext 0.24, the only thing
remaining is the formatting macros (unless you use the `legacy`
feature).

When gettextrs get support for this happens, these crates will be
obsolete.

## License

This crate is licensed under the MIT license.

Author: Hubert Figuière <hub@figuiere.net>
