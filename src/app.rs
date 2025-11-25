// App Module - Main application struct and state management
// This is the core of the egui application
// Manages application state, coordinates between UI and logic layers
// Handles communication with background threads via channels

use eframe::egui;

use crate::ui::main_view;

pub struct App {
    current_view: AppView,
}

pub enum AppView {
    Main,
 
}

impl Default for App {
    fn default() -> Self {
        Self {
            current_view: AppView::Main,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let fps = 1.0 / frame.info().cpu_usage.unwrap_or(0.016);
        egui::CentralPanel::default().show(ctx, |ui| {
            // TODO: Implement main update loop
            ui.label(format!("FPS: {:.2}", fps));
            match self.current_view {
                AppView::Main => main_view::MainView::new().render(ui),
            }
        });
    }
}
