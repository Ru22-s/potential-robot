use gtk4::prelude::*;
use libadwaita::{Application, ApplicationWindow, HeaderBar};
use gtk4::{Box, Orientation, Label, ListBox, ScrolledWindow, Entry, Button, SelectionMode, Align, CssProvider, glib};
use std::rc::Rc;
use std::cell::RefCell;
use async_channel::Sender;

mod ollama;
use ollama::{OllamaClient, ChatMessage};

fn main() {
    // Create Tokio runtime
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = rt.enter();

    let app = Application::builder()
        .application_id("com.llamachat.ui")
        .build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    app.run();
}

fn load_css() {
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
    gtk4::style_context_add_provider_for_display(
        &gtk4::gdk::Display::default().expect("Could not connect to a display."),
        &provider,
        gtk4::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

fn build_ui(app: &Application) {
    let content = Box::new(Orientation::Vertical, 0);

    let header = HeaderBar::new();
    content.append(&header);

    // Messages area
    let messages_list = ListBox::new();
    messages_list.set_selection_mode(SelectionMode::None);
    messages_list.add_css_class("boxed-list");
    messages_list.set_vexpand(true);

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never)
        .vexpand(true)
        .child(&messages_list)
        .build();

    content.append(&scrolled_window);

    // Input area
    let input_box = Box::new(Orientation::Horizontal, 12);
    input_box.set_margin_start(12);
    input_box.set_margin_end(12);
    input_box.set_margin_top(12);
    input_box.set_margin_bottom(12);

    let entry = Entry::builder()
        .placeholder_text("Type a message...")
        .hexpand(true)
        .build();

    let send_button = Button::builder()
        .label("Send")
        .css_classes(vec!["suggested-action"])
        .build();

    input_box.append(&entry);
    input_box.append(&send_button);

    content.append(&input_box);

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Llama Chat UI")
        .content(&content)
        .default_width(800)
        .default_height(600)
        .build();

    window.present();

    // State
    let history = Rc::new(RefCell::new(Vec::<ChatMessage>::new()));
    let client = Rc::new(OllamaClient::new("http://localhost:11434".to_string()));

    // Channel for async -> main communication
    let (sender, receiver) = async_channel::unbounded::<String>();

    // Spawn a local future on the main context to update UI
    let history_clone = history.clone();
    let messages_list_clone = messages_list.clone();
    let scrolled_window_clone = scrolled_window.clone();

    glib::spawn_future_local(async move {
        while let Ok(response) = receiver.recv().await {
            // Add AI response to UI
            add_message(&messages_list_clone, &response, false);
            // Add to history
            history_clone.borrow_mut().push(ChatMessage {
                role: "assistant".to_string(),
                content: response,
            });

            // Scroll to bottom
            scroll_to_bottom(&scrolled_window_clone);
        }
    });

    // Send logic function
    let handle_send: Rc<dyn Fn(&Entry, &ListBox, &Rc<RefCell<Vec<ChatMessage>>>, &Rc<OllamaClient>, &Sender<String>, &ScrolledWindow)> = Rc::new(move |entry, messages_list, history, client, sender, scrolled_window| {
        let text = entry.text();
        if text.is_empty() {
            return;
        }
        entry.set_text("");

        // Add user message to UI
        add_message(messages_list, &text, true);
        scroll_to_bottom(scrolled_window);

        // Add to history
        let mut hist = history.borrow_mut();
        hist.push(ChatMessage {
            role: "user".to_string(),
            content: text.to_string(),
        });

        // Prepare data for async task
        let messages = hist.clone();
        let client_inner = client.as_ref().clone();
        let sender_clone = sender.clone();

        // Spawn async task
        tokio::spawn(async move {
            match client_inner.chat("llama3", messages).await {
                Ok(response) => {
                    sender_clone.send(response).await.expect("Could not send response");
                }
                Err(e) => {
                    sender_clone.send(format!("Error: {}", e)).await.expect("Could not send error");
                }
            }
        });
    });

    let entry_1 = entry.clone();
    let list_1 = messages_list.clone();
    let hist_1 = history.clone();
    let client_1 = client.clone();
    let sender_1 = sender.clone();
    let scroll_1 = scrolled_window.clone();
    let handle_send_1 = handle_send.clone();

    send_button.connect_clicked(move |_| {
        handle_send_1(&entry_1, &list_1, &hist_1, &client_1, &sender_1, &scroll_1);
    });

    let entry_2 = entry.clone();
    let list_2 = messages_list.clone();
    let hist_2 = history.clone();
    let client_2 = client.clone();
    let sender_2 = sender.clone();
    let scroll_2 = scrolled_window.clone();
    let handle_send_2 = handle_send.clone();

    entry.connect_activate(move |_| {
        handle_send_2(&entry_2, &list_2, &hist_2, &client_2, &sender_2, &scroll_2);
    });
}

fn add_message(list: &ListBox, text: &str, is_user: bool) {
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

fn scroll_to_bottom(scrolled_window: &ScrolledWindow) {
    let adj = scrolled_window.vadjustment();
    let adj_clone = adj.clone();
    glib::idle_add_local_once(move || {
        adj_clone.set_value(adj_clone.upper() - adj_clone.page_size());
    });
}
