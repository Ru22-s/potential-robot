use gtk4::prelude::*;
use gtk4::{
    Box, Button, DropDown, Entry, ListBox, Orientation, PolicyType, ScrolledWindow, SelectionMode,
    StringList,
};
use libadwaita::{Application, ApplicationWindow, Clamp, HeaderBar, Toast, ToastOverlay};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use super::widgets::{add_message_row, add_typing_row, scroll_to_bottom};
use crate::api::{ChatMessage, OllamaClient};

enum UIEvent {
    Response { text: String, is_error: bool },
    ModelsLoaded(Vec<String>),
}

pub fn build_ui(app: &Application) {
    let toast_overlay = ToastOverlay::new();

    let main_box = Box::new(Orientation::Vertical, 0);

    let header = HeaderBar::new();

    // Model Selector DropDown
    let initial_models = vec!["llama3".to_string()];
    let model_list_store = StringList::new(&["llama3"]);
    let model_dropdown = DropDown::new(Some(model_list_store.clone()), None::<gtk4::Expression>);
    model_dropdown.set_tooltip_text(Some("Select Ollama Model"));
    header.pack_start(&model_dropdown);

    // Clear Chat Button
    let clear_button = Button::builder()
        .icon_name("user-trash-symbolic")
        .tooltip_text("Clear chat history")
        .build();
    header.pack_end(&clear_button);

    main_box.append(&header);

    // Messages area with Clamp for adaptive width
    let messages_list = ListBox::new();
    messages_list.set_selection_mode(SelectionMode::None);
    messages_list.add_css_class("boxed-list");
    messages_list.set_vexpand(true);

    let messages_clamp = Clamp::builder()
        .maximum_size(800)
        .tightening_threshold(600)
        .child(&messages_list)
        .build();

    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(PolicyType::Never)
        .vexpand(true)
        .child(&messages_clamp)
        .build();

    main_box.append(&scrolled_window);

    // Input area with Clamp
    let input_box = Box::new(Orientation::Horizontal, 8);
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

    let input_clamp = Clamp::builder()
        .maximum_size(800)
        .tightening_threshold(600)
        .child(&input_box)
        .build();

    main_box.append(&input_clamp);

    toast_overlay.set_child(Some(&main_box));

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Llama Chat UI")
        .content(&toast_overlay)
        .default_width(850)
        .default_height(650)
        .build();

    window.present();

    // Application state
    let history = Rc::new(RefCell::new(Vec::<ChatMessage>::new()));
    let client = Arc::new(OllamaClient::new("http://localhost:11434"));
    let available_models = Rc::new(RefCell::new(initial_models));

    // Current typing indicator row, if any
    let typing_row = Rc::new(RefCell::new(None::<gtk4::ListBoxRow>));

    // Communication channel
    let (sender, receiver) = async_channel::unbounded::<UIEvent>();

    // Fetch models on startup
    {
        let client_clone = client.clone();
        let sender_clone = sender.clone();
        tokio::spawn(async move {
            if let Ok(models) = client_clone.list_models().await {
                if !models.is_empty() {
                    let _ = sender_clone.send(UIEvent::ModelsLoaded(models)).await;
                }
            }
        });
    }

    // Event Loop on GTK Main Context
    {
        let history_clone = history.clone();
        let messages_list_clone = messages_list.clone();
        let scrolled_window_clone = scrolled_window.clone();
        let entry_clone = entry.clone();
        let send_button_clone = send_button.clone();
        let typing_row_clone = typing_row.clone();
        let toast_overlay_clone = toast_overlay.clone();
        let model_list_store_clone = model_list_store.clone();
        let model_dropdown_clone = model_dropdown.clone();
        let available_models_clone = available_models.clone();

        glib::spawn_future_local(async move {
            while let Ok(event) = receiver.recv().await {
                match event {
                    UIEvent::ModelsLoaded(models) => {
                        *available_models_clone.borrow_mut() = models.clone();
                        // Re-populate DropDown StringList
                        model_list_store_clone.splice(
                            0,
                            model_list_store_clone.n_items(),
                            &models.iter().map(|s| s.as_str()).collect::<Vec<&str>>(),
                        );
                        // Default to first item or 'llama3' if present
                        if let Some(idx) = models.iter().position(|m| m.contains("llama3")) {
                            model_dropdown_clone.set_selected(idx as u32);
                        } else {
                            model_dropdown_clone.set_selected(0);
                        }
                    }
                    UIEvent::Response { text, is_error } => {
                        // Remove typing row indicator
                        if let Some(row) = typing_row_clone.borrow_mut().take() {
                            messages_list_clone.remove(&row);
                        }

                        // Re-enable inputs
                        entry_clone.set_sensitive(true);
                        send_button_clone.set_sensitive(true);
                        entry_clone.grab_focus();

                        if is_error {
                            let toast = Toast::new(&format!("Ollama Error: {}", text));
                            toast_overlay_clone.add_toast(toast);
                        } else {
                            // Add message to UI & history
                            add_message_row(&messages_list_clone, &text, false);
                            history_clone
                                .borrow_mut()
                                .push(ChatMessage::assistant(text));
                        }

                        scroll_to_bottom(&scrolled_window_clone);
                    }
                }
            }
        });
    }

    // Action: Clear Chat
    {
        let history_clone = history.clone();
        let messages_list_clone = messages_list.clone();
        let toast_overlay_clone = toast_overlay.clone();

        clear_button.connect_clicked(move |_| {
            history_clone.borrow_mut().clear();
            while let Some(child) = messages_list_clone.first_child() {
                messages_list_clone.remove(&child);
            }
            toast_overlay_clone.add_toast(Toast::new("Chat history cleared"));
        });
    }

    // Action: Send Message
    let handle_send = {
        let history = history.clone();
        let client = client.clone();
        let messages_list = messages_list.clone();
        let scrolled_window = scrolled_window.clone();
        let entry = entry.clone();
        let send_button = send_button.clone();
        let model_dropdown = model_dropdown.clone();
        let available_models = available_models.clone();
        let typing_row = typing_row.clone();

        Rc::new(move || {
            let text = entry.text().trim().to_string();
            if text.is_empty() {
                return;
            }

            entry.set_text("");
            entry.set_sensitive(false);
            send_button.set_sensitive(false);

            // Add user message to UI & history
            add_message_row(&messages_list, &text, true);
            let mut hist = history.borrow_mut();
            hist.push(ChatMessage::user(&text));

            // Show typing indicator row
            let t_row = add_typing_row(&messages_list);
            *typing_row.borrow_mut() = Some(t_row);

            scroll_to_bottom(&scrolled_window);

            // Get selected model name
            let selected_index = model_dropdown.selected() as usize;
            let models = available_models.borrow();
            let selected_model = models
                .get(selected_index)
                .cloned()
                .unwrap_or_else(|| "llama3".to_string());

            let messages_payload = hist.clone();
            let client_inner = client.as_ref().clone();
            let sender_clone = sender.clone();

            tokio::spawn(async move {
                match client_inner.chat(&selected_model, messages_payload).await {
                    Ok(response) => {
                        let _ = sender_clone
                            .send(UIEvent::Response {
                                text: response,
                                is_error: false,
                            })
                            .await;
                    }
                    Err(err) => {
                        let _ = sender_clone
                            .send(UIEvent::Response {
                                text: err.to_string(),
                                is_error: true,
                            })
                            .await;
                    }
                }
            });
        })
    };

    // Connect Entry activate (Enter key) and Send button click
    {
        let handle_send_1 = handle_send.clone();
        send_button.connect_clicked(move |_| {
            handle_send_1();
        });

        let handle_send_2 = handle_send.clone();
        entry.connect_activate(move |_| {
            handle_send_2();
        });
    }
}
