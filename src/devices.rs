use std::{io, net::IpAddr};

use regex::Regex;
use thiserror::Error;

use crate::connection::messages::set_pilot::SetPilotRequestBuilder;

use super::color::RGBCW;
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
            mac: system_config.result().mac().to_owned(),
            kind: DeviceKind::from_module_name(system_config.result().module_name())?,
            connection,
        })
    }

    pub fn set_pilot(self) -> SetPilotBuilder {
        SetPilotBuilder {
            device: self,
            request_builder: SetPilotRequestBuilder::new(),
        }
    }
}

pub struct SetPilotBuilder {
    device: Device,
    request_builder: SetPilotRequestBuilder,
}

impl SetPilotBuilder {
    pub fn send(self) -> Result<Device, DeviceError> {
        self.device
            .connection
            .set_pilot(&self.device.ip, self.request_builder.build())
            .map_err(|e| DeviceError::SetPilotError(e))?;
        Ok(self.device)
    }

    pub fn on(mut self) -> Self {
        self.request_builder = self.request_builder.state(true);
        self
    }

    pub fn off(mut self) -> Self {
        self.request_builder = self.request_builder.state(false);
        self
    }

    pub fn rgbcw(mut self, value: RGBCW) -> Result<Self, DeviceError> {
        match self.device.kind {
            DeviceKind::Plug => Err(DeviceError::UnsupportedCommand(
                self.device.kind,
                "setting color".to_string(),
            )),
            DeviceKind::Bulb(ref bulb_kind) => match bulb_kind {
                BulbKind::Color => {
                    self.request_builder = self
                        .request_builder
                        .r(*value.r())
                        .g(*value.g())
                        .b(*value.b())
                        .c(*value.c())
                        .w(*value.w());
                    Ok(self)
                }
                _ => Err(DeviceError::UnsupportedCommand(
                    self.device.kind,
                    "setting color".to_string(),
                )),
            },
        }
    }

    pub fn brightness(mut self, value: u8) -> Result<Self, DeviceError> {
        match self.device.kind {
            DeviceKind::Plug => Err(DeviceError::UnsupportedCommand(
                self.device.kind,
                "setting brightness".to_string(),
            )),
            DeviceKind::Bulb(_) => {
                self.request_builder = self.request_builder.dimming(value);
                Ok(self)
            }
        }
    }
}

#[derive(Debug)]
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

#[derive(Debug)]
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
    SetPilotError(#[source] ConnectionError),
    #[error("{0:?} devices do not support {1}!")]
    UnsupportedCommand(DeviceKind, String),
}
