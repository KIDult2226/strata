# Keyboard shortcuts

This document defines how Strata turns a key press into a shortcut on every
keyboard layout, and how code must read key input. It covers every surface that
handles keys: browser windows, the file chooser, previews, Settings, and dialogs.
What individual keys do is documented with each feature, for example in
[keyboard navigation](keyboard-navigation.md) and [10xer mode](10xer-mode.md).

## How a key press becomes a shortcut

Shortcuts follow the key's label first, then its physical position:

- **Letters** use the letter the active layout prints on the key. On French
  AZERTY, German QWERTZ, or Dvorak, **Ctrl+Z** is the key labelled Z.
- **Non-Latin layouts.** When the key types a character outside printable ASCII
  (Cyrillic, Greek, Hebrew, Arabic, accented letters, and so on), the shortcut
  uses the Latin character that the same physical key types in another
  configured layout, at the same Shift level. If no configured layout has one,
  it uses the key's US-QWERTY character. With a Russian layout active, the key
  that types **с** is **Ctrl+C**, and in 10xer mode the key that types **о**
  is **j**.
- **The active layout's own ASCII character wins** over the fallback. On a
  Russian layout the key that types `.` is **Ctrl+.**; the key that types `ю`
  sits on the US `.` position and also resolves to **Ctrl+.**.
- **AltGr** (third- and fourth-level) characters are text input and never fall
  back.
- **The top digit row is 1–0 by position** on every layout, with or without
  Shift. **Ctrl+1**, **Ctrl+Shift+1** and 10xer digits work on AZERTY, where the
  row types `&é"'(` without Shift, and on layouts whose Shift+digit symbols
  differ from US. Keypad digits count as digits when Num Lock is on.
- **Caps Lock and Num Lock never change a shortcut.**
- **Shift** is part of a letter shortcut: **Ctrl+Shift+K** is not **Ctrl+K**.
  For symbols, the Shift needed to type the symbol is ignored, so **Ctrl++**
  matches whether or not the layout needs Shift for `+`, and **Ctrl+.** works on
  AZERTY, where `.` needs Shift.
- **Modifiers match exactly** over Ctrl, Alt, Shift and Super. **Ctrl+C** does
  not fire for **Ctrl+Alt+C**.
- **Typing is unaffected.** Type-to-search, filter fields, rename fields, and
  footer prompts receive the character the layout produced.
- **Text fields** follow the same rules for copy, cut, paste, select all, undo,
  and redo.
- **Input methods** for Korean, Japanese, and Chinese run on a Latin layout and
  pass Ctrl shortcuts through, so the same rules apply without special cases.

## Shortcut labels

Every place that shows a shortcut renders it from the binding the app matches:
the F1 reference, Settings, context menus, and footer hints. A displayed
shortcut is therefore always one that works.

- **Labels show the Latin key**: **Ctrl+C** with a Russian layout active,
  **Ctrl+1** on AZERTY. Keyboards for non-Latin layouts print the Latin letters
  too, and AZERTY keyboards print the digits, so the label is on the key to
  press. Labels do not change when the active layout changes.
- **Key names follow the interface language**, the way keyboards sold for it are
  printed: German shows **Strg+C** and **Entf**, French **Suppr**, Spanish
  **Supr**. Languages whose keyboards print English names keep them.
- **One notation everywhere.** Modifiers appear in the order Ctrl, Alt, Shift,
  Super. Arrows are ↑ ↓ ← →, and other keys use their full names (Enter,
  Escape, Delete, Page Up). After a modifier, letters are uppercase and Shift is
  spelled out (**Ctrl+Shift+N**). Without a modifier, a letter appears as typed
  (**j**) and Shift+letter as the uppercase letter (**J**), the 10xer
  convention. Chords are separated by a space (**g h**).

## Reading key input in code

### One entry point

`ui::keys` (`src/ui/keys.rs`) is the only module that reads GDK key events.
Install handlers with `keys::on_press(&controller, |press| …)` and
`keys::on_release(…)`. Each event is normalized once into a `KeyPress` and passed
to the handler.

`clippy.toml` lists `EventControllerKey::connect_key_pressed`,
`EventControllerKey::connect_key_released`, and
`GtkApplicationExt::set_accels_for_action` under `disallowed-methods`. `ui::keys` calls them under a single
`#[expect(clippy::disallowed_methods, reason = "…")]`; no other module may.

### `KeyPress`

- `is(binding)` reports whether the press matches a `Binding`. Use it for
  every command, including navigation keys such as Escape, Return, and arrows.
- `digit(modifiers)` returns the digit-row or keypad digit when the modifiers
  match exactly, for ordinal commands such as view modes and tab numbers.
- `text()` returns the typed character, for typing consumers only.
- `is_modifier()` reports a bare modifier key press.

### `Binding` and the binding catalog

A `Binding` is a canonical key plus exact modifiers: `Binding::plain(Key::j)`,
`Binding::ctrl(Key::c)`, `Binding::ctrl_shift(Key::k)`, or
`Binding::new(key, modifiers)`. Letters are lowercase with Shift explicit.
Symbol bindings name the symbol (`Key::plus`), never the key that types it on a
US keyboard.

Every shortcut is a named constant in the binding catalog,
`ui::keys::bindings`, grouped by area: `pub const COPY: Binding =
Binding::ctrl(Key::c);`. Handlers match catalog constants, and the F1 reference
(`ui/shortcut_reference.rs`), context-menu hints, and footer hints refer to the
same constants. Dispatch logic stays in the handlers; the catalog only defines
keys.

What a reference row or hint shows is a `Keys` value, never free text:

- one binding: **Ctrl+C**
- alternatives: **F2 / Ctrl+R**
- a chord: **g h**
- a digit range with its modifiers: **Ctrl+Shift+1–9 / 0**

### Labels

`keys::label` is the only code that turns bindings into text. Key names come
from one translated table (`keys.name.*` in every catalog). Context templates
such as `%{keys} in header` stay translatable and receive the formatted label.

Menus display the formatter's text too. GTK formats a menu item's `accel`
attribute itself: it puts Shift first, capitalizes letters, and uses its own
translations. So menu rows must not show GTK's rendering of an `accel` string,
and no code may build an accelerator string by parsing a label.

### Normalization

1. Start from `gdk::KeyEvent::match_()`, GTK's accelerator form: Caps Lock is
   ignored, modifiers consumed to produce the key are removed, and letters are
   lowercased with Shift added back. Keep only Ctrl, Alt, Shift and Super.
2. If that key is a character outside printable ASCII and the event level is 0
   or 1, replace it with the first printable-ASCII key that
   `Display::map_keycode` reports for another layout at the same level, then
   canonicalize it the same way. Otherwise use the built-in US-QWERTY table
   entry for the keycode. GDK keycodes are evdev codes plus 8 on both X11 and
   Wayland. If neither source has a Latin key, keep the original.
3. Digit-row keycodes 10–19 map to 1–9 and 0 regardless of the key they type.
   `digit()` compares the event's modifier state before consumed modifiers are
   removed, so **Ctrl+1** and **Ctrl+Shift+1** stay distinct on every layout.

The common path costs one `match_()` call and a range check. `map_keycode`
runs only for keys outside ASCII, and nothing is cached, so no keymap-change
listeners are needed.

### Window actions

Search (**Ctrl+K**), jump to folder (**Ctrl+Shift+K**), open terminal
(**Ctrl+Alt+T**), refresh (**F5**), and arrow scope (**Ctrl+\\**) are bindings in
the window dispatcher, not GTK application accelerators. The dispatcher checks
them where GTK used to run its accelerators, after its own stages yield, and
activates the same guarded `win.*` actions. `add_guarded_window_action` still
decides whether they run while a modal or an inline edit owns input, and 10xer
mode availability is a dispatcher check.

### Text fields

When a `gtk::Text` or `gtk::TextView` has focus and a press matched an editing
binding only through the layout fallback, the dispatcher activates the widget's
own action (`clipboard.cut`, `clipboard.copy`, `clipboard.paste`,
`selection.select-all`, `text.undo`, `text.redo`) and stops the event. Otherwise
GTK's own text bindings handle it.

## Adding or changing a shortcut

1. Add the binding to `ui::keys::bindings` and match it in the handler with
   `press.is(…)` or `press.digit(…)`.
2. Reference the same constant from its F1 row in `ui/shortcut_reference.rs`
   and from any context hint. The label renders itself; translate only the
   action text, in every catalog (see
   [internationalization](internationalization.md)).
3. Never compare `text()` or a raw keyval to recognize a command, never type a
   key label by hand, and never add a second spelling (an uppercase letter or a
   US shifted symbol) to cover another layout.
4. Extend the normalizer table tests when a binding uses a symbol or digit, and
   add end-to-end coverage for a new input route.

## Testing

`ui::keys::tests` drives the pure normalizer from hand-written keymap rows for
US, Russian, Ukrainian, Greek, Hebrew, French AZERTY, German QWERTZ, Dvorak,
Caps Lock, and a Russian-only keymap. They also cover label rendering
(notation, translated key names, chords, digit ranges) and catalog completeness:
every catalog binding appears in at least one F1 scope (mode, 10xer, chooser) or
is explicitly marked as unlisted. Run them with
`./scripts/test-headless.py keys::`, and run `./scripts/test-headless.py i18n::`
after changing key names.

End-to-end scenarios switch layouts on the harness's private Xvfb: `setxkbmap`
loads the layouts and an XKB group lock selects the active one. Never change the
desktop's layout from a test. See [end-to-end testing](e2e-testing.md).
