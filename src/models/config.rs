// Config Model - Application configuration structure
// Represents the application's configuration state
// Can be serialized/deserialized to/from TOML files

use std::collections::HashMap;

use crate::utils::vectores_strings::get_iis_security_features;

pub struct Config {

    pub app_settings: AppSettings,
}

pub struct AppSettings {
    pub server_permits_enabled: bool,
    pub server_ddl_enabled: bool,
    pub net_version: String,
    pub enabled_lua: String,
    
    pub set_security_checkboxes: HashMap<&'static str, bool>,

}


impl Config {

    pub fn default() -> Self {
        let mut set_security_checkboxes = HashMap::new();
        for (id, _) in get_iis_security_features() {
            set_security_checkboxes.insert(id, false);
        }

        Self {
            app_settings: AppSettings {
                server_permits_enabled: false,
                server_ddl_enabled: false,
                net_version: "Select .NET version".to_string(),
                enabled_lua: "EnabledLua value".to_string(),
                set_security_checkboxes
            },
        }
    }

}
