
use std::collections::HashMap;

use eframe::egui;


use crate::{components::{
    checkbox::Checkbox, selected::Selected,
}, models::config::{AppSettings, Config}, utils::vectores_strings::{get_enabled_lua_options, get_iis_security_features, get_net_versions}};


pub struct MainView {
    server_settings: AppSettings,
    checkbox_permits: Checkbox,
    checkbox_ddl: Checkbox,
    selected_net_version : Selected,
    selected_value_enabledlua: Selected,
    security_checkboxes : HashMap<&'static str, Checkbox>,
}

impl MainView {
    pub fn new() -> Self {
        let mut security_checkboxes = HashMap::new();

        for (id, label) in get_iis_security_features() {
            security_checkboxes.insert(
                id,
                Checkbox::new(label, egui::Id::new(id)),
            );
        }

        Self {
            server_settings: Config::default().app_settings,
            checkbox_permits: Checkbox::new("Enable Server Permits", egui::Id::new("checkbox_permits")),
            checkbox_ddl: Checkbox::new("Enable Server DDL", egui::Id::new("checkbox_ddl")),
            selected_net_version: Selected::new("Selected .NET version", get_net_versions(), egui::Id::new("selected_net_version")),
            selected_value_enabledlua: Selected::new("EnabledLua Options", get_enabled_lua_options(), egui::Id::new("selected_value_enabledlua")),
            security_checkboxes
        }
    }

    pub fn render(&mut self, ui: &mut egui::Ui) {
        ui.heading("General Settings");
        self.checkbox_permits.render(ui, &mut self.server_settings.server_permits_enabled);
        self.checkbox_ddl.render(ui, &mut self.server_settings.server_ddl_enabled);
        self.selected_net_version.render(ui, &mut self.server_settings.net_version);
        self.selected_value_enabledlua.render(ui, &mut self.server_settings.enabled_lua);
        
        ui.separator();
        
        // Web Server Security Section
        ui.heading("Web Server Security Features");
        ui.label("Configure Windows Server IIS Security features:");
        
        ui.spacing();
        
        // Use stable iteration order from the original array
        for (id, _label) in get_iis_security_features() {
            if let Some(checkbox) = self.security_checkboxes.get_mut(id) {
                if let Some(value) = self.server_settings.set_security_checkboxes.get_mut(id) {
                    checkbox.render(ui, value);
                }
            }
        }

        ui.separator();

        if ui.button("Apply Settings").on_hover_text("Apply the current settings").clicked() {
            self.on_click();
        }
    }

    pub fn on_click(&self) {
        
    }
 

    
}