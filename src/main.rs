#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use gtk4::prelude::*;
use libadwaita::Application;

mod api;
mod ui;

fn main() {
    // Create Tokio runtime
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");
    let _guard = rt.enter();

    let app = Application::builder()
        .application_id("com.llamachat.ui")
        .build();

    app.connect_startup(|_| ui::load_css());
    app.connect_activate(ui::build_ui);
    app.run();
}
