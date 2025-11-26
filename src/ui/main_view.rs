
use std::{collections::HashMap, sync::mpsc::Receiver};

use eframe::egui;


use crate::{components::{
    checkbox::Checkbox, selected::Selected,
}, logic::iis_security::IISSecurityLogic, models::config::{AppSettings, Config}, services::command_executor::CommandStatus, utils::vectores_strings::{get_enabled_lua_options, get_iis_security_features, get_net_versions}};


pub struct MainView {
    server_settings: AppSettings,
    checkbox_permits: Checkbox,
    checkbox_ddl: Checkbox,
    selected_net_version : Selected,
    selected_value_enabledlua: Selected,
    security_checkboxes : HashMap<&'static str, Checkbox>,


    //asyn operations 
    command_reciver: Option<Receiver<CommandStatus>>,
    last_status: Option<CommandStatus>,
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
            security_checkboxes,
            command_reciver: None,
            last_status: None,
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

        // Check for command status updates
        if let Some(receiver) = &self.command_reciver {
            if let Ok(status) = receiver.try_recv() {
                self.last_status = Some(status);
                ui.ctx().request_repaint(); 
            }
        }

        if let Some(status) = &self.last_status {
            ui.separator();
            match status {
                CommandStatus::Running(msg) => {
                    ui.label(format!("⏳ {}", msg));
                    ui.spinner(); // Shows loading spinner
                },
                CommandStatus::Success(msg) => {
                    ui.colored_label(egui::Color32::GREEN, format!("✓ Success"));
                    ui.label(msg);
                },
                CommandStatus::Error(msg) => {
                    ui.colored_label(egui::Color32::RED, format!("✗ Error"));
                    ui.label(msg);
                },
            }
        }
    }

    pub fn on_click(&mut self) {
        println!("Button clicked!");
        println!("Raw net_version value: {}", self.server_settings.net_version);
        
        // Check if user selected a valid version
        if self.server_settings.net_version.starts_with("Select") || 
           self.server_settings.net_version.is_empty() {
            println!("No valid .NET version selected!");
            self.last_status = Some(CommandStatus::Error(
                "Please select a .NET Framework version first".to_string()
            ));
            return;
        }
        
        // Extract version number (e.g., ".NET Framework 4.8" -> "48")
        let version_net_framework = self.server_settings.net_version
            .split_whitespace()
            .last()
            .unwrap_or("45")
            .replace(".", "");
        
        println!("Parsed .NET version: {}", version_net_framework);
        println!("Starting async command...");
        
        let receiver = IISSecurityLogic::install_net_framework_async(version_net_framework);
        self.command_reciver = Some(receiver);
        self.last_status = Some(CommandStatus::Running("Initializing...".to_string()));
        
        println!("Command started, receiver stored");
    }
    
}