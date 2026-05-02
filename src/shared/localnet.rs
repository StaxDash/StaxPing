use get_if_addrs::get_if_addrs;

pub struct LocalNetInfo {
    pub ipv4: Option<String>,
    pub interface: Option<String>,
    pub gateway: Option<String>, // placeholder for future gateway detection
}

pub fn get_local_info() -> LocalNetInfo {
    let mut ipv4 = None;
    let mut interface = None;

    if let Ok(addrs) = get_if_addrs() {
        for iface in addrs {
            if iface.is_loopback() {
                continue;
            }

            if let std::net::IpAddr::V4(ip) = iface.ip() {
                ipv4 = Some(ip.to_string());
                interface = Some(iface.name);
                break;
            }
        }
    }

    LocalNetInfo {
        ipv4,
        interface,
        gateway: None, // gateway detection coming later
    }
}
