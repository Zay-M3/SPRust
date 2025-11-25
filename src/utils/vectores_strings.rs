
//This is a vector for the .net versions
pub fn get_net_versions() -> Vec<String>{
    vec![
        "net6.0".to_string(),
        "net7.0".to_string(),
        "net8.0".to_string(),
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