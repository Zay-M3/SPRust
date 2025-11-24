
use eframe::egui;

pub fn render(ui: &mut egui::Ui, label: &str) {

    let button = ui.add(egui::Button::new(label));
    if button.clicked() {
        // Handle button click event
        println!("{} button clicked", label);
    }
}