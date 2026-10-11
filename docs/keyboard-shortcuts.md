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

The F1 reference and Settings name keys by their US labels.

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

### `Binding`

A `Binding` is a canonical key plus exact modifiers: `Binding::plain(Key::j)`,
`Binding::ctrl(Key::c)`, `Binding::ctrl_shift(Key::k)`, or
`Binding::new(key, modifiers)`. Declare bindings as named constants beside the
handler that owns them, for example `const COPY: Binding = Binding::ctrl(Key::c);`.
Letters are lowercase with Shift explicit. Symbol bindings name the symbol
(`Key::plus`), never the key that types it on a US keyboard.

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

1. Declare a `Binding` constant beside its handler and match it with
   `press.is(…)` or `press.digit(…)`.
2. Never compare `text()` or a raw keyval to recognize a command, and never add
   a second spelling (an uppercase letter or a US shifted symbol) to cover
   another layout.
3. Add or update the row in `ui/shortcut_reference.rs` and translate its text in
   every catalog (see [internationalization](internationalization.md)).
4. Extend the normalizer table tests when a binding uses a symbol or digit, and
   add end-to-end coverage for a new input route.

## Testing

`ui::keys::tests` drives the pure normalizer from hand-written keymap rows for
US, Russian, Ukrainian, Greek, Hebrew, French AZERTY, German QWERTZ, Dvorak,
Caps Lock, and a Russian-only keymap. Run them with
`./scripts/test-headless.py keys::`.

End-to-end scenarios switch layouts on the harness's private Xvfb: `setxkbmap`
loads the layouts and an XKB group lock selects the active one. Never change the
desktop's layout from a test. See [end-to-end testing](e2e-testing.md).
