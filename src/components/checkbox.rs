use eframe::egui;

pub struct Checkbox {
    label: String,
    id: egui::Id,
}

impl Checkbox {

    pub fn new(label: &str, id: egui::Id) -> Self {
        Self {
            label: label.to_string(), 
            id
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui, value: &mut bool) {
        let mut checked = ui.data_mut(|data| {
            data.get_temp::<bool>(self.id).unwrap_or(*value)
        });
        ui.checkbox(&mut checked, &self.label);

        ui.data_mut(|data| {
            data.insert_temp(self.id, checked);
        });

        *value = checked;
    }


}
