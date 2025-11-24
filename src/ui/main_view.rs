use eframe::egui;
use crate::components::*;


pub fn render(ui: &mut egui::Ui) {
    // Example usage of custom components
    checkbox::render(ui, "Aplly permits to server");
    checkbox::render(ui, "Enable DEP");
    button_submit::render(ui, "Apply Changes");
}