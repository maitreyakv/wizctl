use std::{io, net::IpAddr};

use regex::Regex;
use thiserror::Error;

use super::connection::{Connection, ConnectionError};

pub struct Device {
    ip: IpAddr,
    mac: String,
    kind: DeviceKind,
    connection: Connection,
}

impl Device {
    pub fn connect(ip: IpAddr) -> Result<Self, DeviceError> {
        let connection = Connection::new().map_err(|e| DeviceError::ClientInitError(e))?;
        let system_config = connection
            .get_system_config(&ip)
            .map_err(|e| DeviceError::ConnectError(e))?;
        Ok(Self {
            ip,
            mac: system_config.mac().to_owned(),
            kind: DeviceKind::from_module_name(system_config.module_name())?,
            connection,
        })
    }

    pub fn turn_on(&self) -> Result<(), DeviceError> {
        self.connection
            .turn_device_on(&self.ip)
            .map_err(|e| DeviceError::StateChangeError(e))
    }

    pub fn turn_off(&self) -> Result<(), DeviceError> {
        self.connection
            .turn_device_off(&self.ip)
            .map_err(|e| DeviceError::StateChangeError(e))
    }
}

enum DeviceKind {
    Plug,
    Bulb(BulbKind),
}

impl DeviceKind {
    fn from_module_name(module_name: &str) -> Result<Self, DeviceError> {
        let identifier = Regex::new(r"^ESP\d{2}_(\w+)_\d{2}[ABIT]*$")
            .expect("Failed to compile regex!")
            .captures(module_name)
            .map(|capture| capture.get(0))
            .flatten()
            .ok_or_else(|| DeviceError::UnrecognizedModuleName(module_name.to_string()))?
            .as_str();

        if identifier.contains("SOCKET") {
            Ok(DeviceKind::Plug)
        } else if identifier.contains("TW") {
            Ok(DeviceKind::Bulb(BulbKind::TunableWhite))
        } else if identifier.contains("DW") {
            Ok(DeviceKind::Bulb(BulbKind::DimmableWhite))
        } else if identifier.contains("RGB") {
            Ok(DeviceKind::Bulb(BulbKind::Color))
        } else {
            Err(DeviceError::UnrecognizedModuleName(module_name.to_string()))
        }
    }
}

enum BulbKind {
    DimmableWhite,
    TunableWhite,
    Color,
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Client failed to initialize!")]
    ClientInitError(#[source] io::Error),
    #[error("Failed to connect to device!")]
    ConnectError(#[source] ConnectionError),
    #[error("Did not recognize module name: {0}!")]
    UnrecognizedModuleName(String),
    #[error("Failed to change the state of a device!")]
    StateChangeError(#[source] ConnectionError),
}
