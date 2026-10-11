// SPDX-License-Identifier: MIT

use gtk::gdk::Key;

use crate::ui::keys::Binding;

catalog! {
    listed {
        CHOOSE_FOLDER: Binding = Binding::ctrl(Key::Return);
    }
    unlisted {}
}
