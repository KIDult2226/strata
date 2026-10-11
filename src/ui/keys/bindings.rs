// SPDX-License-Identifier: MIT

//! Every shortcut as a named constant, grouped by area. Handlers match these
//! constants, and the F1 reference, hints and labels show the same ones.

use super::{Binding, Digits};

/// Declares an area's constants and lists them for the catalog tests.
/// Constants under `unlisted` need not appear in the F1 reference.
macro_rules! catalog {
    (
        listed {
            $($(#[$meta:meta])* $name:ident: $kind:ident = $value:expr;)*
        }
        unlisted {
            $($(#[$umeta:meta])* $uname:ident: $ukind:ident = $uvalue:expr;)*
        }
    ) => {
        $($(#[$meta])* pub(crate) const $name: $kind = $value;)*
        $($(#[$umeta])* pub(crate) const $uname: $ukind = $uvalue;)*

        #[cfg(test)]
        pub(super) const LISTED: &[(&str, super::Entry)] =
            &[$((stringify!($name), super::Entry::$kind($name))),*];
        #[cfg(test)]
        pub(super) const UNLISTED: &[(&str, super::Entry)] =
            &[$((stringify!($uname), super::Entry::$ukind($uname))),*];
    };
}

mod chooser;
mod dialogs;
mod navigation;
mod preview;
mod reference;
mod tenxer;
mod widgets;
mod window;

pub(crate) use chooser::*;
pub(crate) use dialogs::*;
pub(crate) use navigation::*;
pub(crate) use preview::*;
pub(crate) use reference::*;
pub(crate) use tenxer::*;
pub(crate) use widgets::*;
pub(crate) use window::*;

#[cfg(test)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Entry {
    Binding(Binding),
    Digits(Digits),
}

#[cfg(test)]
pub(super) fn listed() -> impl Iterator<Item = (&'static str, Entry)> {
    [
        chooser::LISTED,
        dialogs::LISTED,
        navigation::LISTED,
        preview::LISTED,
        reference::LISTED,
        tenxer::LISTED,
        widgets::LISTED,
        window::LISTED,
    ]
    .into_iter()
    .flatten()
    .copied()
}

#[cfg(test)]
pub(super) fn unlisted() -> impl Iterator<Item = (&'static str, Entry)> {
    [
        chooser::UNLISTED,
        dialogs::UNLISTED,
        navigation::UNLISTED,
        preview::UNLISTED,
        reference::UNLISTED,
        tenxer::UNLISTED,
        widgets::UNLISTED,
        window::UNLISTED,
    ]
    .into_iter()
    .flatten()
    .copied()
}
