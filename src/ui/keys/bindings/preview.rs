// SPDX-License-Identifier: MIT

use gtk::gdk::Key;

use crate::ui::keys::Binding;

catalog! {
    listed {
        MEDIA_PLAY_PAUSE: Binding = Binding::ctrl_alt(Key::space);
        MEDIA_SEEK_BACK: Binding = Binding::ctrl_alt(Key::Left);
        MEDIA_SEEK_FORWARD: Binding = Binding::ctrl_alt(Key::Right);
        MEDIA_VOLUME_UP: Binding = Binding::ctrl_alt(Key::Up);
        MEDIA_VOLUME_DOWN: Binding = Binding::ctrl_alt(Key::Down);
        MEDIA_MUTE: Binding = Binding::ctrl_alt(Key::m);
        MEDIA_PREVIOUS: Binding = Binding::ctrl_alt(Key::less);
        MEDIA_NEXT: Binding = Binding::ctrl_alt(Key::greater);
    }
    tenxer_only {
        TENXER_MEDIA_MUTE: Binding = Binding::plain(Key::m);
        TENXER_MEDIA_PREVIOUS: Binding = Binding::plain(Key::less);
        TENXER_MEDIA_NEXT: Binding = Binding::plain(Key::greater);
        SCROLL_PREVIEW_DOWN: Binding = Binding::shift(Key::j);
        SCROLL_PREVIEW_UP: Binding = Binding::shift(Key::k);
        TENXER_FIND_IN_PREVIEW: Binding = Binding::plain(Key::slash);
    }
    unlisted {
        FIND_IN_PREVIEW: Binding = Binding::ctrl(Key::f);
        PREVIOUS_MATCH: Binding = Binding::shift(Key::Return);

        // GTK's own text keys, which a focused document keeps.
        DOCUMENT_START: Binding = Binding::ctrl(Key::Home);
        DOCUMENT_END: Binding = Binding::ctrl(Key::End);
    }
}
