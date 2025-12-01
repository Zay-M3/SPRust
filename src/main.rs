
// Module declarations - tells Rust about the folder structure
mod app;
mod ui;
mod components;
mod logic;
mod services;
mod models;
mod utils;
mod hooks;

use eframe::egui;

fn main() -> eframe::Result<()> {
    // TODO: Initialize logging
    // env_logger::init();
    
    // Configure window options
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([500.0, 600.0])
            .with_min_inner_size([500.0, 600.0]),
        ..Default::default()
    };

    // Start the application
    eframe::run_native(
        "SPRust - System Performance/Windows Parametrization",
        options,
        Box::new(|_cc| Box::new(app::App::default())),
    )
}
