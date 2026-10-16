use gtk4::gdk::Display;
use gtk4::{style_context_add_provider_for_display, CssProvider, STYLE_PROVIDER_PRIORITY_APPLICATION};

pub fn load_css() {
    let provider = CssProvider::new();
    provider.load_from_string("
        .user-bubble {
            background-color: #3584e4;
            color: white;
            border-radius: 15px;
            padding: 10px;
            margin: 5px;
        }
        .bot-bubble {
            background-color: #424242;
            color: white;
            border-radius: 15px;
            padding: 10px;
            margin: 5px;
        }
        entry {
            border-radius: 10px;
        }
    ");

    if let Some(display) = Display::default() {
        style_context_add_provider_for_display(
            &display,
            &provider,
            STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}
