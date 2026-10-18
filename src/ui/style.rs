use gtk4::gdk::Display;
use gtk4::{
    style_context_add_provider_for_display, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION,
};

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string(
        "
        .user-bubble {
            background-color: var(--accent-bg-color, #3584e4);
            color: var(--accent-fg-color, #ffffff);
            border-radius: 16px;
            padding: 10px 14px;
            margin: 4px 0px;
        }
        .bot-bubble {
            background-color: var(--card-bg-color, #303030);
            color: var(--card-fg-color, #ffffff);
            border-radius: 16px;
            padding: 10px 14px;
            margin: 4px 0px;
        }
        .message-row {
            padding: 2px 8px;
        }
        .message-label {
            font-size: 14px;
            line-height: 1.4;
        }
    ",
    );

    if let Some(display) = Display::default() {
        style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
