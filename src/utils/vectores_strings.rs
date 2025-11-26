
//This is a vector for the .NET Framework versions (for Windows Server Features)
pub fn get_net_versions() -> Vec<String>{
    vec![
        ".NET Framework 3.5".to_string(),
        ".NET Framework 4.5".to_string(),
        ".NET Framework 4.6".to_string(),
        ".NET Framework 4.7".to_string(),
        ".NET Framework 4.8".to_string(),
    ]
}

pub fn get_enabled_lua_options() -> Vec<String>{
    vec![
        "1".to_string(),
        "0".to_string(),
    ]
}

pub fn get_iis_security_features() -> Vec<(&'static str, &'static str)> {
    vec![
        ("request_filtering", "Request Filtering (Web-Request-Filtering)"),
        ("basic_auth", "Basic Authentication (Web-Basic-Auth)"),
        ("cert_provider", "Centralized SSL Certificate Support (Web-CertProvider)"),
        ("client_cert_auth", "Client Certificate Mapping Authentication (Web-Client-Auth)"),
        ("digest_auth", "Digest Authentication (Web-Digest-Auth)"),
        ("iis_client_cert_map", "IIS Client Certificate Mapping Authentication (Web-Cert-Mapping)"),
        ("ip_domain_restrictions", "IP and Domain Restrictions (Web-IP-Security)"),
        ("url_authorization", "URL Authorization (Web-Url-Auth)"),
        ("windows_auth", "Windows Authentication (Web-Windows-Auth)"),
    ]
}