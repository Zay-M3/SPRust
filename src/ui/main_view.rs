
use std::collections::HashMap;

use eframe::egui;


use crate::{components::{
    checkbox::{Checkbox}, selected::Selected,
}, utils::vectores_strings::{get_enabled_lua_options, get_iis_security_features, get_net_versions}};

pub struct AppSettings {
    server_permits_enabled: bool,
    server_ddl_enabled: bool,
    net_version: String,
    enabled_lua: String,
    
    request_filtering: bool,
    basic_auth: bool,
    ssl_cert_support: bool,
    client_cert_mapping_auth: bool,
    digest_auth: bool,
    iis_client_cert_mapping: bool,
    ip_domain_restrictions: bool,
    url_authorization: bool,
    windows_auth: bool,
}


pub struct MainView {
    server_settings: AppSettings,
    checkbox_permits: Checkbox,
    checkbox_ddl: Checkbox,
    selected_net_version : Selected,
    selected_value_enabledlua: Selected,
    
    // Web Server Security Feature Checkboxes
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
            server_settings: AppSettings {
                server_permits_enabled: false,
                server_ddl_enabled: false,
                net_version: "Select .NET version".to_string(),
                enabled_lua: "EnabledLua value".to_string(),
                
                request_filtering: true,
                basic_auth: true,
                ssl_cert_support: true,
                client_cert_mapping_auth: true,
                digest_auth: true,
                iis_client_cert_mapping: true,
                ip_domain_restrictions: false,  
                url_authorization: true,
                windows_auth: true,
            },
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
        
        self.security_checkboxes.get_mut("request_filtering").unwrap().render(ui, &mut self.server_settings.request_filtering);
        self.security_checkboxes.get_mut("basic_auth").unwrap().render(ui, &mut self.server_settings.basic_auth);
        self.security_checkboxes.get_mut("cert_provider").unwrap().render(ui, &mut self.server_settings.ssl_cert_support);
        self.security_checkboxes.get_mut("client_cert_auth").unwrap().render(ui, &mut self.server_settings.client_cert_mapping_auth);
        self.security_checkboxes.get_mut("digest_auth").unwrap().render(ui, &mut self.server_settings.digest_auth);
        self.security_checkboxes.get_mut("iis_client_cert_map").unwrap().render(ui, &mut self.server_settings.iis_client_cert_mapping);
        self.security_checkboxes.get_mut("ip_domain_restrictions").unwrap().render(ui, &mut self.server_settings.ip_domain_restrictions);
        self.security_checkboxes.get_mut("url_authorization").unwrap().render(ui, &mut self.server_settings.url_authorization);
        self.security_checkboxes.get_mut("windows_auth").unwrap().render(ui, &mut self.server_settings.windows_auth);

        ui.separator();

        if ui.button("Apply Settings").on_hover_text("Apply the current settings").clicked() {
            self.on_click();
        }
    }

    pub fn on_click(&self) {
        println!("=== Settings Applied ===");
        println!("\n--- General Settings ---");
        println!("Server Permits Enabled: {}", self.server_settings.server_permits_enabled);
        println!("Server DDL Enabled: {}", self.server_settings.server_ddl_enabled);
        println!(".NET Version: {}", self.server_settings.net_version);
        println!("EnabledLua Option: {}", self.server_settings.enabled_lua);
        
        println!("\n--- Web Server Security Features ---");
        println!("Request Filtering: {}", self.server_settings.request_filtering);
        println!("Basic Authentication: {}", self.server_settings.basic_auth);
        println!("Centralized SSL Certificate Support: {}", self.server_settings.ssl_cert_support);
        println!("Client Certificate Mapping Authentication: {}", self.server_settings.client_cert_mapping_auth);
        println!("Digest Authentication: {}", self.server_settings.digest_auth);
        println!("IIS Client Certificate Mapping: {}", self.server_settings.iis_client_cert_mapping);
        println!("IP and Domain Restrictions: {}", self.server_settings.ip_domain_restrictions);
        println!("URL Authorization: {}", self.server_settings.url_authorization);
        println!("Windows Authentication: {}", self.server_settings.windows_auth);
        
        println!("\n=== End of Settings ===");
    }
 

    
}