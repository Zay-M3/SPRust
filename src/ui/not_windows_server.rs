pub struct NotWindows;

impl NotWindows {
    pub fn new() -> Self {
        Self {}
    }

    pub fn render(&mut self, ui: &mut eframe::egui::Ui) {
        ui.label("This application is only supported on Windows Server.");
        
    }
}