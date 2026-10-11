// SPDX-License-Identifier: MIT

use gtk::{
    gdk::{Key, ModifierType},
    prelude::*,
};

use super::*;
use crate::ui::{
    keys::{KeyPress, bindings::*},
    preferences::PreferenceManager,
};

const NONE: ModifierType = ModifierType::empty();
const CONTROL: ModifierType = ModifierType::CONTROL_MASK;
const SHIFT: ModifierType = ModifierType::SHIFT_MASK;
const ALT: ModifierType = ModifierType::ALT_MASK;
const SUPER: ModifierType = ModifierType::SUPER_MASK;
const CAPS: ModifierType = ModifierType::LOCK_MASK;

/// A US-QWERTY press as GDK delivers it without an event.
fn press(key: Key, modifiers: ModifierType) -> KeyPress {
    KeyPress::from_signal(key, 0, modifiers)
}

#[test]
fn plain_single_pane_arrows_move_focus_not_directories() {
    let action = |mode, key, modifiers, at_left_edge, sidebar_visible| {
        single_pane_arrow_action(mode, &press(key, modifiers), at_left_edge, sidebar_visible)
    };
    assert_eq!(
        action(BrowserMode::Icons, Key::Left, NONE, false, true),
        Some(SinglePaneArrow::Native)
    );
    for mode in [BrowserMode::Icons, BrowserMode::List] {
        assert_eq!(
            action(mode, Key::Left, NONE, true, true),
            Some(SinglePaneArrow::Sidebar)
        );
        for key in [Key::Up, Key::Down] {
            assert_eq!(
                action(mode, key, NONE, true, true),
                Some(SinglePaneArrow::Native)
            );
        }
        for key in [Key::Left, Key::Right, Key::Up] {
            assert_eq!(action(mode, key, ALT, true, true), None);
        }
        assert_eq!(action(mode, Key::Return, NONE, true, true), None);
    }
    assert_eq!(
        action(BrowserMode::List, Key::Right, NONE, true, true),
        Some(SinglePaneArrow::Stay)
    );
    assert_eq!(
        action(BrowserMode::List, Key::Left, NONE, true, false),
        Some(SinglePaneArrow::Stay)
    );
    assert_eq!(
        action(BrowserMode::Icons, Key::Left, NONE, true, false),
        Some(SinglePaneArrow::Native)
    );
    assert_eq!(
        action(BrowserMode::Columns, Key::Left, NONE, true, true),
        None
    );
    for modifier in [SHIFT, CONTROL] {
        assert_eq!(
            action(BrowserMode::Icons, Key::Left, modifier, true, true),
            Some(SinglePaneArrow::Native)
        );
    }
}

#[test]
fn sidebar_arrows_and_vim_keys_share_focus_directions() {
    for (arrow, vim) in [
        (Key::Left, Key::h),
        (Key::Right, Key::l),
        (Key::Up, Key::k),
        (Key::Down, Key::j),
    ] {
        assert_eq!(
            sidebar_focus_direction(&press(arrow, NONE), NONE),
            vim_focus_direction(&press(vim, NONE))
        );
    }
    assert_eq!(
        sidebar_focus_direction(&press(Key::Return, NONE), NONE),
        None
    );
    assert_eq!(sidebar_focus_direction(&press(Key::Up, SHIFT), NONE), None);
    assert_eq!(
        sidebar_focus_direction(&press(Key::Up, SHIFT), SHIFT),
        Some(gtk::DirectionType::Up)
    );
}

#[test]
fn navigation_keys_claim_keyboard_ownership_but_commands_do_not() {
    for key in [
        Key::Up,
        Key::Down,
        Key::h,
        Key::j,
        Key::k,
        Key::l,
        Key::Tab,
        Key::ISO_Left_Tab,
        Key::Page_Down,
        Key::Home,
        Key::End,
        Key::KP_Home,
        Key::KP_End,
        Key::Return,
    ] {
        assert!(is_browser_navigation_key(&press(key, NONE)), "{key:?}");
    }
    assert!(is_browser_navigation_key(&press(Key::Down, SHIFT)));
    assert!(is_browser_navigation_key(&press(Key::Left, ALT)));
    for key in [Key::v, Key::c, Key::x, Key::z, Key::Control_L, Key::Delete] {
        assert!(!is_browser_navigation_key(&press(key, CONTROL)), "{key:?}");
    }
}

#[test]
fn mouse_history_buttons_map_to_navigation_actions() {
    assert_eq!(mouse_history_action(8), Some(MouseHistoryAction::Back));
    assert_eq!(mouse_history_action(9), Some(MouseHistoryAction::Forward));
    for button in [1, 2, 3, 4, 5, 6, 7, 10] {
        assert_eq!(mouse_history_action(button), None);
    }
}

#[test]
fn open_terminal_shortcut_requires_control_alt() {
    assert!(press(Key::t, CONTROL | ALT).is(TERMINAL));
    assert!(press(Key::t, CONTROL | ALT | CAPS).is(TERMINAL));
    assert!(!press(Key::t, NONE).is(TERMINAL));
    assert!(!press(Key::t, CONTROL | SHIFT).is(TERMINAL));
    assert!(!press(Key::t, CONTROL).is(TERMINAL));
    assert!(!press(Key::t, CONTROL | ALT | SHIFT).is(TERMINAL));
    assert!(!press(Key::F4, CONTROL).is(TERMINAL));
}

#[test]
fn undo_shortcut_requires_control_alone() {
    assert!(press(Key::z, CONTROL).is(UNDO));
    assert!(press(Key::z, CONTROL | CAPS).is(UNDO));
    assert!(!press(Key::z, NONE).is(UNDO));
    assert!(!press(Key::z, CONTROL | SHIFT).is(UNDO));
    assert!(!press(Key::z, CONTROL | ALT).is(UNDO));
    assert!(!press(Key::z, CONTROL | SUPER).is(UNDO));
}

#[test]
fn redo_shortcut_accepts_control_shift_z_and_control_y() {
    let redo = |key, modifiers| press(key, modifiers).is_any(&[REDO, REDO_Y]);
    assert!(redo(Key::z, CONTROL | SHIFT));
    assert!(redo(Key::Z, CONTROL | SHIFT));
    assert!(redo(Key::y, CONTROL));
    assert!(redo(Key::y, CONTROL | CAPS));
    assert!(!redo(Key::z, CONTROL));
    assert!(!redo(Key::z, CONTROL | SHIFT | ALT));
    assert!(!redo(Key::y, CONTROL | SHIFT));
    assert!(!redo(Key::y, CONTROL | ALT));
    assert!(!redo(Key::z, SHIFT));
    assert!(!redo(Key::z, NONE));
}

#[test]
fn page_keys_and_their_keypad_twins_are_one_binding() {
    for (key, binding) in [
        (Key::Page_Up, PAGE_UP),
        (Key::KP_Page_Up, PAGE_UP),
        (Key::Page_Down, PAGE_DOWN),
        (Key::KP_Page_Down, PAGE_DOWN),
    ] {
        assert!(press(key, NONE).is(binding), "{key:?}");
    }
    assert!(!press(Key::Home, NONE).is_any(&[PAGE_UP, PAGE_DOWN]));
}

#[test]
fn jump_shortcut_requires_control_without_other_command_modifiers() {
    let jump = [FIRST_ITEM_UP, LAST_ITEM_DOWN];
    assert!(press(Key::Up, CONTROL).is(FIRST_ITEM_UP));
    assert!(press(Key::Down, CONTROL).is(LAST_ITEM_DOWN));
    assert!(!press(Key::Left, CONTROL).is_any(&jump));
    assert!(!press(Key::Up, NONE).is_any(&jump));
    for modifier in [SHIFT, ALT, SUPER] {
        assert!(!press(Key::Up, CONTROL | modifier).is_any(&jump));
    }
}

#[test]
fn toggle_hidden_shortcut_accepts_h_or_period_with_only_control() {
    let hidden =
        |key, modifiers| press(key, modifiers).is_any(&[HIDDEN_FILES, HIDDEN_FILES_PERIOD]);
    assert!(hidden(Key::h, CONTROL));
    assert!(hidden(Key::h, CONTROL | CAPS));
    assert!(hidden(Key::period, CONTROL));
    assert!(!hidden(Key::h, NONE));
    assert!(!hidden(Key::h, CONTROL | SHIFT));
    assert!(!hidden(Key::h, CONTROL | ALT));
    // US Shift turns the period key into `>`.
    assert!(!hidden(Key::greater, CONTROL | SHIFT));
}

#[test]
fn sidebar_focus_shortcut_requires_control_and_shift() {
    assert!(press(Key::b, CONTROL | SHIFT).is(SIDEBAR_FOCUS));
    assert!(press(Key::B, CONTROL | SHIFT).is(SIDEBAR_FOCUS));
    assert!(!press(Key::b, CONTROL).is(SIDEBAR_FOCUS));
}

#[test]
fn context_menu_shortcut_accepts_menu_key_alone_and_shift_f10() {
    let menu = |key, modifiers| press(key, modifiers).is_any(&[CONTEXT_MENU, CONTEXT_MENU_F10]);
    assert!(menu(Key::Menu, NONE));
    assert!(menu(Key::F10, SHIFT));
    assert!(!menu(Key::Menu, SHIFT));
    assert!(!menu(Key::Menu, CONTROL));
    assert!(!menu(Key::F10, NONE));
    assert!(!menu(Key::F10, SHIFT | CONTROL));
    assert!(!menu(Key::F10, SHIFT | ALT));
    assert!(!menu(Key::F10, SHIFT | SUPER));
    assert!(!menu(Key::Menu, SUPER));
    // Hyper, Meta and the locks are not shortcut modifiers.
    for modifier in [ModifierType::HYPER_MASK, ModifierType::META_MASK, CAPS] {
        assert!(menu(Key::F10, SHIFT | modifier));
        assert!(menu(Key::Menu, modifier));
    }
}

#[test]
fn type_to_search_accepts_printable_keys_without_command_modifiers() {
    assert_eq!(
        type_to_search_query(&press(Key::a, NONE)),
        Some(TypeToSearchQuery::Character('a'))
    );
    assert_eq!(
        type_to_search_query(&press(Key::A, SHIFT)),
        Some(TypeToSearchQuery::Character('A'))
    );
    assert_eq!(
        type_to_search_query(&press(Key::period, NONE)),
        Some(TypeToSearchQuery::Character('.'))
    );
}

#[test]
fn type_to_search_uses_slash_to_open_an_empty_filter() {
    assert_eq!(
        type_to_search_query(&press(Key::slash, NONE)),
        Some(TypeToSearchQuery::Empty)
    );
}

#[test]
fn type_to_search_leaves_space_for_quick_preview() {
    for modifiers in [NONE, SHIFT] {
        assert_eq!(type_to_search_query(&press(Key::space, modifiers)), None);
    }
}

#[test]
fn type_to_search_ignores_shortcuts_and_non_printable_keys() {
    assert_eq!(type_to_search_query(&press(Key::k, CONTROL)), None);
    assert_eq!(type_to_search_query(&press(Key::F5, NONE)), None);
}

#[test]
fn vim_focus_keys_map_to_gtk_directions() {
    for (key, direction) in [
        (Key::h, gtk::DirectionType::Left),
        (Key::j, gtk::DirectionType::Down),
        (Key::k, gtk::DirectionType::Up),
        (Key::l, gtk::DirectionType::Right),
    ] {
        assert_eq!(vim_focus_direction(&press(key, NONE)), Some(direction));
    }
    assert_eq!(vim_focus_direction(&press(Key::Down, NONE)), None);
}

#[test]
fn control_digits_select_each_browser_presentation() {
    for (key, mode) in [
        (Key::_1, BrowserMode::Columns),
        (Key::_2, BrowserMode::Icons),
        (Key::_3, BrowserMode::List),
        (Key::KP_3, BrowserMode::List),
    ] {
        assert_eq!(browser_mode_for_digit(&press(key, CONTROL)), Some(mode));
    }
    assert_eq!(browser_mode_for_digit(&press(Key::_4, CONTROL)), None);
    assert_eq!(browser_mode_for_digit(&press(Key::a, CONTROL)), None);
    assert_eq!(browser_mode_for_digit(&press(Key::_1, NONE)), None);
}

#[test]
fn rename_shortcut_accepts_f2_and_control_r() {
    let rename = |key, modifiers| press(key, modifiers).is_any(&[RENAME, RENAME_R]);
    assert!(rename(Key::F2, NONE));
    assert!(rename(Key::r, CONTROL));
    assert!(rename(Key::r, CONTROL | CAPS));
}

#[test]
fn rename_shortcut_ignores_extra_modifiers_and_other_keys() {
    let rename = |key, modifiers| press(key, modifiers).is_any(&[RENAME, RENAME_R]);
    assert!(!rename(Key::r, NONE));
    assert!(!rename(Key::r, CONTROL | SHIFT));
    assert!(!rename(Key::r, CONTROL | ALT));
    assert!(!rename(Key::F2, CONTROL));
    assert!(!rename(Key::F5, CONTROL));
}

#[test]
fn refresh_shortcut_keeps_f5_and_releases_control_r() {
    assert!(press(Key::F5, NONE).is(REFRESH));
    assert!(!press(Key::r, NONE).is(REFRESH));
    assert!(!press(Key::F5, NONE).is_any(&[RENAME, RENAME_R]));
    assert!(press(Key::r, CONTROL).is_any(&[RENAME, RENAME_R]));
}

#[test]
fn default_accels_never_bind_one_chord_twice() {
    gtk_test(
        "ui::window::tests::keyboard_policy::default_accels_never_bind_one_chord_twice",
        || {
            let mut seen = std::collections::HashMap::new();
            for (action, accels) in DEFAULT_ACCELS {
                for accel in *accels {
                    let chord = gtk::accelerator_parse(*accel).expect("valid default accelerator");
                    assert!(
                        !press(chord.0, chord.1).is_any(&[RENAME, RENAME_R]),
                        "{action} must not claim a rename shortcut"
                    );
                    assert!(
                        seen.insert(chord, *action).is_none(),
                        "{accel} is bound to more than one action"
                    );
                }
            }
            let refresh = DEFAULT_ACCELS
                .iter()
                .find(|(action, _)| *action == "win.refresh")
                .expect("refresh accels")
                .1;
            assert_eq!(refresh, &["F5"]);
        },
    );
}

#[test]
fn tenxer_accelerators_follow_the_saved_mode_across_windows() {
    gtk_test(
        "ui::window::tests::keyboard_policy::tenxer_accelerators_follow_the_saved_mode_across_windows",
        || {
            let preferences = PreferenceManager::shared();
            preferences.set_tenxer_mode(false);
            let first = policy_window();
            assert_accelerators(&first, false);
            preferences.set_tenxer_mode(true);
            assert_accelerators(&first, true);
            let enabled = accel_snapshot(&first);
            let second = policy_window();
            assert_eq!(accel_snapshot(&second), enabled);
            preferences.set_tenxer_mode(true);
            assert_eq!(accel_snapshot(&first), enabled);
            first.destroy();
            settle_policy();
            assert_eq!(accel_snapshot(&second), enabled);
            preferences.set_tenxer_mode(false);
            assert_accelerators(&second, false);
            let restored = accel_snapshot(&second);
            preferences.set_tenxer_mode(false);
            assert_eq!(accel_snapshot(&second), restored);
            second.destroy();
        },
    );
}

fn policy_window() -> gtk::ApplicationWindow {
    let preferences = PreferenceManager::shared();
    let application = gtk::gio::Application::default()
        .and_downcast::<gtk::Application>()
        .unwrap_or_else(|| {
            let application =
                gtk::Application::new(None::<&str>, gtk::gio::ApplicationFlags::NON_UNIQUE);
            application
                .register(None::<&gtk::gio::Cancellable>)
                .expect("test application registration");
            application
        });
    let window = gtk::ApplicationWindow::builder()
        .application(&application)
        .default_width(800)
        .default_height(600)
        .build();
    let content = super::super::composition::WindowContent::new(&window, &preferences);
    content.bind(&window, &preferences);
    window.present();
    window
}

fn assert_accelerators(window: &gtk::ApplicationWindow, tenxer: bool) {
    let application = window.application().expect("application");
    for (action, expected) in DEFAULT_ACCELS {
        let installed = application.accels_for_action(action);
        let suppressed = tenxer
            && matches!(
                *action,
                "win.jump-folder" | "win.open-terminal" | "win.toggle-arrow-scope"
            );
        if suppressed {
            assert!(
                installed.is_empty(),
                "{action} stays bound while 10xer is on: {installed:?}"
            );
            continue;
        }
        assert_eq!(installed.len(), expected.len(), "{action}");
        for (actual, &expected) in installed.iter().zip(*expected) {
            assert_eq!(
                gtk::accelerator_parse(actual).expect("installed accelerator"),
                gtk::accelerator_parse(expected).expect("default accelerator"),
                "{action}"
            );
        }
    }
}

fn accel_snapshot(window: &gtk::ApplicationWindow) -> Vec<(String, Vec<String>)> {
    let application = window.application().expect("application");
    DEFAULT_ACCELS
        .iter()
        .map(|(action, _)| {
            (
                (*action).to_owned(),
                application
                    .accels_for_action(action)
                    .iter()
                    .map(|accel| accel.to_string())
                    .collect(),
            )
        })
        .collect()
}

fn settle_policy() {
    let context = gtk::glib::MainContext::default();
    while context.pending() {
        context.iteration(false);
    }
}

#[test]
fn native_editing_shortcuts_are_left_to_the_focused_widget() {
    let editing = [SELECT_ALL, COPY, PASTE, CUT];
    for key in [Key::a, Key::c, Key::v, Key::x] {
        assert!(press(key, CONTROL).is_any(&editing), "{key:?}");
    }
    assert!(!press(Key::c, CONTROL | SHIFT).is_any(&editing));
}
