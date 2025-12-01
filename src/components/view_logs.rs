//This view is for see the logs when the application is runing in realtime mode with a lot of data a bar of process is needed
use eframe::egui;



pub struct ViewLogs;

impl ViewLogs {
    pub fn new() -> Self {
        Self 
    }

    pub fn render(&mut self, ui: &mut egui::Ui, logs: &Vec<String>, progress: f32, is_running: bool) {
        let progress_bar = if is_running {
            egui::ProgressBar::new(progress)
                .text("Processing...")
                .show_percentage()
                .animate(true)  // Shows animated progress
        } else {
            egui::ProgressBar::new(1.0)
                .text("Completed")
                .show_percentage()
        };
        //bar progress for logs
        ui.add(progress_bar);
        ui.add_space(10.0);

        egui::ScrollArea::vertical()
            .max_height(400.0)
            .stick_to_bottom(true) 
            .show(ui, |ui| {
                ui.with_layout(egui::Layout::top_down(egui::Align::LEFT), |ui| {
                    for log in logs {
                        // Color-code logs based on prefix
                        if log.contains("✗") {
                            ui.colored_label(egui::Color32::RED, log);
                        } else if log.contains("→") {
                            ui.colored_label(egui::Color32::GREEN, log);
                        } else if log.contains("•") {
                            ui.colored_label(egui::Color32::YELLOW, log);
                        } else {
                            ui.label(log);
                        }
                    }
                });
            });
    }
    
}