use std::{
    net::{AddrParseError, IpAddr, SocketAddr},
    path::PathBuf,
    str::FromStr,
};

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[non_exhaustive]
pub struct Config {
    pub address: ServiceAddress,
    pub database_url: String,
    pub fs_storage_root_dir: PathBuf,
    pub auth: Auth,
}

#[derive(Deserialize, Debug)]
pub struct Auth {
    pub jwt_secret: String,
    pub jwt_expiration_seconds: i64,
    pub enforce_totp: bool,
}

#[derive(Deserialize, Debug)]
#[serde(try_from = "RawServiceAddress")]
pub struct ServiceAddress(pub SocketAddr);

#[derive(Deserialize)]
pub struct RawServiceAddress {
    host_name: String,
    port: u16,
}

#[derive(thiserror::Error, Debug)]
#[error("Failed to parse host address configuration")]
pub struct HostParseError(#[from] AddrParseError);

impl TryFrom<RawServiceAddress> for ServiceAddress {
    type Error = HostParseError;

    fn try_from(value: RawServiceAddress) -> Result<Self, Self::Error> {
        let ip_address = IpAddr::from_str(&value.host_name)?;

        Ok(ServiceAddress(SocketAddr::new(ip_address, value.port)))
    }
}

impl Config {
    pub fn tracing_level() -> tracing::level_filters::LevelFilter {
        #[cfg(debug_assertions)]
        return tracing::level_filters::LevelFilter::TRACE;
        #[cfg(not(debug_assertions))]
        return tracing::level_filters::LevelFilter::INFO;
    }
}
