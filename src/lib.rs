use std::net::{Ipv4Addr, Ipv6Addr};

use ipnet::Ipv6Net;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub serv: Ipv4Addr,
    pub tn64: Ipv6Addr,
    pub rt64: Ipv6Addr,
    pub rt48: Ipv6Addr,
    pub updt: String,
}

#[derive(Clone, Debug)]
pub struct UsableConfig {
    pub serv: Ipv4Addr,
    pub tn64: Ipv6Net,
    pub rt64: Ipv6Net,
    pub rt48: Ipv6Net,
    pub updt: String,
}

impl From<Config> for UsableConfig {
    fn from(config: Config) -> Self {
        Self {
            serv: config.serv,
            tn64: Ipv6Net::new(config.tn64, 64).unwrap(),
            rt64: Ipv6Net::new(config.rt64, 64).unwrap(),
            rt48: Ipv6Net::new(config.rt48, 48).unwrap(),
            updt: config.updt,
        }
    }
}
