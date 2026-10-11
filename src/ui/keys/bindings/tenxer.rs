// SPDX-License-Identifier: MIT

use gtk::gdk::Key;

use crate::ui::keys::{Binding, Digits};

catalog! {
    listed {
        CLOSE_WINDOW: Binding = Binding::shift(Key::q);
        TENXER_REFERENCE: Binding = Binding::plain(Key::asciitilde);
        TENXER_SIDEBAR: Binding = Binding::ctrl(Key::n);

        OPEN: Binding = Binding::plain(Key::o);
        PEEK: Binding = Binding::plain(Key::i);
        TENXER_LAST_ITEM: Binding = Binding::shift(Key::g);
        TENXER_HISTORY_BACK: Binding = Binding::shift(Key::h);
        TENXER_HISTORY_FORWARD: Binding = Binding::shift(Key::l);
        HALF_PAGE_UP: Binding = Binding::ctrl(Key::u);
        HALF_PAGE_DOWN: Binding = Binding::ctrl(Key::d);
        TENXER_PAGE_UP: Binding = Binding::ctrl(Key::b);
        TENXER_PAGE_DOWN: Binding = Binding::ctrl(Key::f);

        YANK: Binding = Binding::plain(Key::y);
        YANK_CUT: Binding = Binding::plain(Key::x);
        UNYANK: Binding = Binding::shift(Key::y);
        UNYANK_X: Binding = Binding::shift(Key::x);
        PASTE_KEEP_BOTH: Binding = Binding::plain(Key::p);
        PASTE_REPLACE: Binding = Binding::shift(Key::p);
        TENXER_DELETE: Binding = Binding::plain(Key::d);
        TENXER_DELETE_PERMANENTLY: Binding = Binding::shift(Key::d);
        CREATE: Binding = Binding::plain(Key::a);
        TENXER_RENAME: Binding = Binding::plain(Key::r);
        OPEN_WITH: Binding = Binding::shift(Key::o);
        MOVE_TO: Binding = Binding::shift(Key::m);
        COPY_TO: Binding = Binding::shift(Key::c);
        RESTORE: Binding = Binding::shift(Key::r);
        TENXER_HIDDEN_FILES: Binding = Binding::plain(Key::period);

        VISUAL_SELECT: Binding = Binding::plain(Key::v);
        VISUAL_UNSET: Binding = Binding::shift(Key::v);
        INVERT_SELECTION: Binding = Binding::ctrl(Key::r);

        FIND: Binding = Binding::plain(Key::slash);
        FIND_BACKWARD: Binding = Binding::plain(Key::question);
        REPEAT_FIND: Binding = Binding::plain(Key::n);
        REPEAT_FIND_REVERSED: Binding = Binding::shift(Key::n);
        FILTER_LISTING: Binding = Binding::plain(Key::f);
        SEARCH_SUBFOLDERS: Binding = Binding::plain(Key::s);
        JUMP_VISITED: Binding = Binding::plain(Key::z);
        JUMP_RECENT: Binding = Binding::shift(Key::z);

        GO: Binding = Binding::plain(Key::g);
        GO_FIRST: Binding = Binding::plain(Key::g);
        GO_FOLLOW: Binding = Binding::plain(Key::f);
        GO_PROMPT: Binding = Binding::plain(Key::space);
        GO_HOME: Binding = Binding::plain(Key::h);
        GO_CONFIG: Binding = Binding::plain(Key::c);
        GO_DOWNLOADS: Binding = Binding::plain(Key::d);
        GO_DOCUMENTS: Binding = Binding::plain(Key::k);
        GO_MUSIC: Binding = Binding::plain(Key::m);
        GO_PICTURES: Binding = Binding::plain(Key::p);
        GO_VIDEOS: Binding = Binding::plain(Key::v);
        GO_TRASH: Binding = Binding::plain(Key::t);
        GO_NETWORK: Binding = Binding::plain(Key::n);
        GO_RECENT: Binding = Binding::plain(Key::r);
        GO_PIN: Binding = Binding::plain(Key::plus);
        GO_UNPIN: Binding = Binding::plain(Key::minus);
        PIN_NUMBERS: Digits = Digits::after(GO, 9, false);

        SORT_BY: Binding = Binding::plain(Key::comma);
        SORT_BY_NAME: Binding = Binding::plain(Key::a);
        SORT_BY_MODIFIED: Binding = Binding::plain(Key::m);
        SORT_BY_SIZE: Binding = Binding::plain(Key::s);
        SORT_BY_TYPE: Binding = Binding::plain(Key::e);

        COPY_AS: Binding = Binding::plain(Key::c);
        COPY_AS_PATH: Binding = Binding::plain(Key::c);
        COPY_AS_NAME: Binding = Binding::plain(Key::n);

        ACTIONS: Binding = Binding::plain(Key::semicolon);
        ACTIONS_TERMINAL: Binding = Binding::plain(Key::t);
        ACTIONS_COMPRESS: Binding = Binding::plain(Key::c);
        ACTIONS_EXTRACT: Binding = Binding::plain(Key::e);
        ACTIONS_EXTRACT_TO: Binding = Binding::shift(Key::e);
        ACTIONS_NUMBERS: Digits = Digits::after(ACTIONS, 9, true);

        TAB_CHORD: Binding = Binding::plain(Key::t);
        TAB_CHORD_NEW: Binding = Binding::plain(Key::n);
        TAB_CHORD_CLOSE: Binding = Binding::plain(Key::x);
        TAB_CHORD_PREVIOUS: Binding = Binding::plain(Key::t);
        TAB_CHORD_NUMBERS: Digits = Digits::after(TAB_CHORD, 9, true);
    }
    unlisted {
        // Where no tab strip takes them first, as in file choosers, these page too.
        TENXER_PAGE_UP_PAGE_UP: Binding = Binding::ctrl(Key::Page_Up);
        TENXER_PAGE_DOWN_PAGE_DOWN: Binding = Binding::ctrl(Key::Page_Down);

        // The reference shows these as "Shift reverses" beside the sort chord.
        SORT_BY_NAME_REVERSED: Binding = Binding::shift(Key::a);
        SORT_BY_MODIFIED_REVERSED: Binding = Binding::shift(Key::m);
        SORT_BY_SIZE_REVERSED: Binding = Binding::shift(Key::s);
        SORT_BY_TYPE_REVERSED: Binding = Binding::shift(Key::e);
    }
}
