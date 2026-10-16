use gtk4::pango::WrapMode;
use gtk4::prelude::*;
use gtk4::{glib, Align, Box, Label, ListBox, ListBoxRow, Orientation, ScrolledWindow};

/// Adds a message bubble row to the chat ListBox and returns the created ListBoxRow.
pub fn add_message_row(list: &ListBox, text: &str, is_user: bool) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_selectable(false);
    row.set_activatable(false);
    row.add_css_class("message-row");

    let row_box = Box::new(Orientation::Horizontal, 0);

    let bubble = Box::new(Orientation::Horizontal, 0);

    if is_user {
        bubble.add_css_class("user-bubble");
        row_box.set_halign(Align::End);
    } else {
        bubble.add_css_class("bot-bubble");
        row_box.set_halign(Align::Start);
    }

    let label = Label::new(Some(text));
    label.set_wrap(true);
    label.set_wrap_mode(WrapMode::WordChar);
    label.set_selectable(true);
    label.set_xalign(0.0);
    label.add_css_class("message-label");

    bubble.append(&label);
    row_box.append(&bubble);
    row.set_child(Some(&row_box));

    list.append(&row);
    row
}

/// Adds a temporary typing indicator row to the chat ListBox.
pub fn add_typing_row(list: &ListBox) -> ListBoxRow {
    let row = ListBoxRow::new();
    row.set_selectable(false);
    row.set_activatable(false);
    row.add_css_class("message-row");

    let row_box = Box::new(Orientation::Horizontal, 0);
    row_box.set_halign(Align::Start);

    let bubble = Box::new(Orientation::Horizontal, 8);
    bubble.add_css_class("bot-bubble");

    let spinner = gtk4::Spinner::new();
    spinner.start();

    let label = Label::new(Some("Thinking..."));
    label.add_css_class("message-label");

    bubble.append(&spinner);
    bubble.append(&label);
    row_box.append(&bubble);
    row.set_child(Some(&row_box));

    list.append(&row);
    row
}

/// Scrolls the scrolled window to the very bottom smoothly.
pub fn scroll_to_bottom(scrolled_window: &ScrolledWindow) {
    let adj = scrolled_window.vadjustment();
    let adj_clone = adj.clone();
    glib::idle_add_local_once(move || {
        adj_clone.set_value(adj_clone.upper() - adj_clone.page_size());
    });
}
