use gtk4::prelude::*;
use gtk4::{Align, Box, Label, ListBox, Orientation, ScrolledWindow, glib};

pub fn add_message_row(list: &ListBox, text: &str, is_user: bool) {
    let row_box = Box::new(Orientation::Horizontal, 0);
    row_box.set_margin_top(4);
    row_box.set_margin_bottom(4);
    row_box.set_margin_start(12);
    row_box.set_margin_end(12);

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
    label.set_max_width_chars(50);
    label.set_xalign(0.0);

    bubble.append(&label);
    row_box.append(&bubble);
    list.append(&row_box);
}

pub fn scroll_to_bottom(scrolled_window: &ScrolledWindow) {
    let adj = scrolled_window.vadjustment();
    let adj_clone = adj.clone();
    glib::idle_add_local_once(move || {
        adj_clone.set_value(adj_clone.upper() - adj_clone.page_size());
    });
}
