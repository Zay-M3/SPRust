
use std::{collections::HashMap, sync::mpsc::Receiver};

use eframe::egui;


use crate::
    {components::{
    checkbox::Checkbox, selected::Selected, view_logs::ViewLogs}, 
    logic::iis_security::IISSecurityLogic, models::config::{AppSettings, Config}, 
    utils::vectores_strings::{get_enabled_lua_options, get_iis_security_features, get_net_versions},
    hooks::{commands::{clean_commands_queue, commands_queue_receiver, execute_commands_queue}, command_executor::CommandStatus
    },};


pub struct MainView {
    server_settings: AppSettings,
    checkbox_permits: Checkbox,
    checkbox_ddl: Checkbox,
    selected_net_version : Selected,
    selected_value_enabledlua: Selected,
    security_checkboxes : HashMap<&'static str, Checkbox>,


    //asyn operations 
    last_status: Option<CommandStatus>,
    active_command: Vec<(String, Receiver<CommandStatus>)>,
    logs: Vec<String>,
    is_running: bool,
    view_logs: ViewLogs,
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
            last_status: None,
            logs: Vec::new(),
            is_running: false,
            view_logs: ViewLogs::new(),
            active_command: Vec::new(),
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

        let mut completed = Vec::new();

        // Execute commands queue
        execute_commands_queue(
            &mut self.active_command,
            &mut self.last_status,
            &mut self.logs,
            &mut completed,
            ui
        );


        // Remove completed commands
        clean_commands_queue(&mut self.active_command, &completed);

        if !self.logs.is_empty() {
            ui.separator();
            ui.heading("Logs:");
            
            let progress = if self.is_running {
                (ui.input(|i| i.time) % 2.0 / 2.0) as f32 
            } else {
                1.0
            };

            self.view_logs.render(ui, &self.logs, progress, self.is_running);

            if ui.button("Clear Logs").clicked() {
                self.logs.clear();
            }
        }

        if self.active_command.is_empty() && self.is_running {
            self.is_running = false;
            self.logs.push("All operations completed.".to_string());
        }
    }

    pub fn on_click(&mut self) {
        self.logs.clear();
        self.logs.push("Starting installation...".to_string());
        
        // Check if user selected a valid version
        if self.server_settings.net_version.starts_with("Select") || 
            self.server_settings.net_version.is_empty() {
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
        
        let receiver = IISSecurityLogic::install_net_framework_async(version_net_framework);
        self.active_command.push((".NET Framework Installation".to_string(), receiver));
            
        commands_queue_receiver(
            &mut self.active_command,
            &self.server_settings
        );
        
        self.last_status = Some(CommandStatus::Running("Initializing...".to_string()));
        self.is_running = true;
    }
    
}