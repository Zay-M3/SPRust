use eframe::egui;

pub struct Selected {
    label: String,
    options: Vec<String>,
    id: egui::Id,
}


impl Selected {
    pub fn new(label: &str, options: Vec<String>, id: egui::Id) -> Self {
        Self {
            label: label.to_string(),
            options,
            id,
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, selected: &mut String) {
        
        let mut current = ui.data_mut(|data| {
            data.get_temp::<String>(self.id)
                .unwrap_or(selected.clone())
        });

        egui::ComboBox::from_label(&self.label)
            .selected_text(&current)
            .show_ui(ui, |ui| {
                for option in &self.options {
                    ui.selectable_value(&mut current, option.clone(), option.as_str());
                }
            });

        ui.data_mut(|data| {
            data.insert_temp(self.id, current.clone());
        });

        *selected = current;

    }

}