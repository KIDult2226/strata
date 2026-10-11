// SPDX-License-Identifier: MIT
use gtk::{glib, prelude::*};
use std::time::Duration;

const NOTICE_CLASS: &str = "overlay-notice";

/// Shows a brief non-interactive notice at the top of `overlay`, replacing any
/// notice still visible there.
pub(super) fn show_notice(
    overlay: &gtk::Overlay,
    heading: Option<&str>,
    text: &str,
    duration: Duration,
) {
    let mut child = overlay.first_child();
    while let Some(widget) = child {
        child = widget.next_sibling();
        if widget.has_css_class(NOTICE_CLASS) {
            overlay.remove_overlay(&widget);
        }
    }
    let row = gtk::Box::new(gtk::Orientation::Horizontal, 8);
    row.add_css_class(NOTICE_CLASS);
    row.set_halign(gtk::Align::Center);
    row.set_valign(gtk::Align::Start);
    row.set_can_target(false);
    let icon = crate::assets::primary_icon(crate::assets::icons::CHECK, 16);
    icon.set_valign(gtk::Align::Start);
    icon.set_can_target(false);
    row.append(&icon);
    let column = gtk::Box::new(gtk::Orientation::Vertical, 2);
    if let Some(heading) = heading {
        let heading = gtk::Label::new(Some(heading));
        heading.add_css_class("overlay-notice-heading");
        heading.set_xalign(0.0);
        column.append(&heading);
    }
    let label = gtk::Label::new(Some(text));
    label.set_xalign(0.0);
    label.set_wrap(true);
    label.set_wrap_mode(gtk::pango::WrapMode::WordChar);
    label.set_max_width_chars(60);
    label.set_lines(3);
    label.set_ellipsize(gtk::pango::EllipsizeMode::End);
    column.append(&label);
    column.set_can_target(false);
    row.append(&column);
    overlay.add_overlay(&row);
    let weak_overlay = overlay.downgrade();
    let weak_row = row.downgrade();
    glib::timeout_add_local_once(duration, move || {
        let (Some(overlay), Some(row)) = (weak_overlay.upgrade(), weak_row.upgrade()) else {
            return;
        };
        if row.parent().as_ref() == Some(overlay.upcast_ref()) {
            overlay.remove_overlay(&row);
        }
    });
}
