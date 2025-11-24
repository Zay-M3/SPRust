use eframe::egui;

pub struct Checkbox {
    label: String,
    checked: bool,
    id: egui::Id,
}

impl Checkbox {
    pub fn new(label: &str, checked: bool, id: egui::Id) -> Self {
        Self {
            label: label.to_string(),
            checked,
            id
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        self.checked = ui.data_mut(|data| {
            data.get_temp::<bool>(self.id).unwrap_or(self.checked)
        });
        ui.checkbox(&mut self.checked, &self.label);

        ui.data_mut(|data| {
            data.insert_temp(self.id, self.checked);
        });
    }

}

pub fn render(ui: &mut egui::Ui, label: &str) {

    let mut checkbox = Checkbox::new(label, false, ui.make_persistent_id(label));
    checkbox.render(ui);
}