//this function is for detected the system where .exe is executed, if not is windows server return None



pub fn is_windows_server() -> Option<String> {
    // Implementation to detect if the system is Windows Server
    // Return Some(String) if it is Windows Server, otherwise None
    #[cfg(target_os = "windows")]
    {
        use sysinfo::System;
        
        // In sysinfo 0.30, use name() and long_os_version()
        let os_name = System::name().unwrap_or_default();
        let os_version = System::long_os_version().unwrap_or_default();
        
        // Check if it's Windows Server
        if os_name.contains("Windows") && (
            os_version.contains("Server") || 
            os_version.contains("server")
        ) {
            Some(format!("{} {}", os_name, os_version))
        } else {
            None
        }
    }
    #[cfg(not(target_os = "windows"))]
    {
        None
    }
}