
use std::sync::mpsc::Receiver;
use crate::hooks::command_executor::{CommandExecutor, CommandStatus};

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

    pub fn enable_basic_authentication() -> Receiver<CommandStatus> {
        // PowerShell command to enable Basic Authentication
        let script = format!(
            "Install-WindowsFeature -Name Web-Basic-Auth"
        );
        let description = String::from("Enable Basic Authentication");

        CommandExecutor::execute_powershell_async(script, description)
    }

    pub fn enable_cert_provider() -> Receiver<CommandStatus> {
        // PowerShell command to enable Centralized SSL Certificate Support
        let script = format!(
            "Install-WindowsFeature -Name Web-CertProvider"
        );
        let description = String::from("Enable Centralized SSL Certificate Support");

        CommandExecutor::execute_powershell_async(script, description)
    }

    pub fn enable_client_cert_authentication() -> Receiver<CommandStatus> {
        // PowerShell command to enable Client Certificate Mapping Authentication
        let script = format!(
            "Install-WindowsFeature -Name Web-Client-Auth"
        );
        let description = String::from("Enable Client Certificate Mapping Authentication");

        CommandExecutor::execute_powershell_async(script, description)
    }

    pub fn enable_digest_authentication() -> Receiver<CommandStatus> {
        // PowerShell command to enable Digest Authentication
        let script = format!(
            "Install-WindowsFeature -Name Web-Digest-Auth"
        );
        let description = String::from("Enable Digest Authentication");

        CommandExecutor::execute_powershell_async(script, description)
    }

    pub fn enable_iis_client_cert_mapping() -> Receiver<CommandStatus> {
        // PowerShell command to enable IIS Client Certificate Mapping Authentication
        let script = format!(
            "Install-WindowsFeature -Name Web-IIS-ClientCertMap"
        );
        let description = String::from("Enable IIS Client Certificate Mapping Authentication");

        CommandExecutor::execute_powershell_async(script, description)
    }
    

    
} 