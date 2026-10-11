// SPDX-License-Identifier: MIT

use gtk::gdk::Key;

use crate::ui::keys::Binding;

catalog! {
    listed {
        ESCAPE: Binding = Binding::plain(Key::Escape);
    }
    unlisted {}
}
