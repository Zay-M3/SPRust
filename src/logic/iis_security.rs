
use std::process::Command;
use std::sync::mpsc::Receiver;
use crate::services::command_executor::{CommandExecutor, CommandStatus};

pub struct IISSecurityLogic;


impl IISSecurityLogic {
    pub fn install_net_framework_async(version: String) -> Receiver<CommandStatus> {
        // Logic to install .NET Framework
        let script = format!(
            "Install-WindowsFeature -Name NET-Framework-{} -IncludeAllSubFeature -IncludeManagementTools", 
            version
        );
        let description = format!(".NET Framework {} Installation", version);

        CommandExecutor::execute_powershell_async(script, description)
    }

    pub fn enable_request_filtering() -> Receiver<CommandStatus> {
        // PowerShell command to enable Request Filtering
        let script = format!(
            "Install-WindowsFeature -Name Web-Filtering"
        );
        let description = String::from("Enable Request Filtering");

        CommandExecutor::execute_powershell_async(script, description)
    }

    
} 