// SPDX-License-Identifier: MIT

use gtk::gdk::Key;

use crate::ui::keys::Binding;

catalog! {
    listed {
        COPY: Binding = Binding::ctrl(Key::c);
        CUT: Binding = Binding::ctrl(Key::x);
        PASTE: Binding = Binding::ctrl(Key::v);
        SELECT_ALL: Binding = Binding::ctrl(Key::a);
        UNDO: Binding = Binding::ctrl(Key::z);
        REDO: Binding = Binding::ctrl_shift(Key::z);
        REDO_Y: Binding = Binding::ctrl(Key::y);
    }
    unlisted {}
}
