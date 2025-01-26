use std::{fmt::Display, io, net::IpAddr};

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
    pub fn discover() -> Result<Vec<Self>, DeviceError> {
        let connection = Connection::new().map_err(DeviceError::ClientInitError)?;
        connection
            .discover()
            .map_err(DeviceError::ConnectError)?
            .into_iter()
            .map(|(ip, system_config)| {
                Ok(Self {
                    ip,
                    mac: system_config.result().mac().to_string(),
                    kind: DeviceKind::from_module_name(system_config.result().module_name())?,
                    connection: Connection::new().map_err(DeviceError::ClientInitError)?,
                })
            })
            .collect()
    }

    pub fn connect(ip: IpAddr) -> Result<Self, DeviceError> {
        let connection = Connection::new().map_err(DeviceError::ClientInitError)?;
        let system_config = connection
            .get_system_config(&ip)
            .map_err(DeviceError::ConnectError)?;
        Ok(Self {
            ip,
            mac: system_config.result().mac().to_owned(),
            kind: DeviceKind::from_module_name(system_config.result().module_name())?,
            connection,
        })
    }

    pub fn ip(&self) -> &IpAddr {
        &self.ip
    }

    pub fn mac(&self) -> &str {
        &self.mac
    }

    pub fn kind(&self) -> &DeviceKind {
        &self.kind
    }

    pub fn set_pilot(self) -> SetPilotBuilder {
        SetPilotBuilder {
            device: self,
            request_builder: SetPilotRequestBuilder::new(),
        }
    }

    pub fn get_rssi(&self) -> Result<i8, DeviceError> {
        Ok(self
            .connection
            .get_pilot(&self.ip)
            .map_err(DeviceError::ConnectError)?
            .result()
            .rssi()
            .to_owned())
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
            .map_err(DeviceError::SetPilotError)?;
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
        if !self.device.kind.is_color() {
            Err(DeviceError::UnsupportedCommand(
                self.device.kind,
                "setting color".to_string(),
            ))
        } else {
            self.request_builder = self
                .request_builder
                .r(*value.r())
                .g(*value.g())
                .b(*value.b())
                .c(*value.c())
                .w(*value.w());
            Ok(self)
        }
    }

    pub fn brightness(mut self, value: u8) -> Result<Self, DeviceError> {
        if !self.device.kind.is_dimmable() {
            Err(DeviceError::UnsupportedCommand(
                self.device.kind,
                "setting brightness".to_string(),
            ))
        } else {
            self.request_builder = self.request_builder.dimming(value);
            Ok(self)
        }
    }
}

#[derive(Debug)]
pub enum DeviceKind {
    Plug,
    LightStrip,
    Bulb(BulbKind),
}

impl Display for DeviceKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plug => "Plug".to_string(),
                Self::LightStrip => "Light Strip".to_string(),
                Self::Bulb(bulb_kind) => format!("{} Bulb", bulb_kind),
            }
        )
    }
}

impl DeviceKind {
    fn from_module_name(module_name: &str) -> Result<Self, DeviceError> {
        let identifier = Regex::new(r"^ESP\d{2}_(\w+)_\d{2}[ABIT]*$")
            .expect("Failed to compile regex!")
            .captures(module_name)
            .and_then(|capture| capture.get(0))
            .ok_or_else(|| DeviceError::UnrecognizedModuleName(module_name.to_string()))?
            .as_str();

        if identifier.contains("SOCKET") {
            Ok(Self::Plug)
        } else if identifier.contains("TW") {
            Ok(Self::Bulb(BulbKind::TunableWhite))
        } else if identifier.contains("DW") {
            Ok(Self::Bulb(BulbKind::DimmableWhite))
        } else if identifier.contains("RGB") {
            if module_name.ends_with("ABI") {
                Ok(Self::LightStrip)
            } else {
                Ok(Self::Bulb(BulbKind::Color))
            }
        } else {
            Err(DeviceError::UnrecognizedModuleName(module_name.to_string()))
        }
    }

    fn is_dimmable(&self) -> bool {
        match self {
            Self::Plug => false,
            Self::Bulb(_) | Self::LightStrip => true,
        }
    }

    fn is_color(&self) -> bool {
        match self {
            Self::Plug => false,
            Self::LightStrip => true,
            Self::Bulb(bulb_kind) => bulb_kind.is_color(),
        }
    }
}

#[derive(Debug)]
pub enum BulbKind {
    DimmableWhite,
    TunableWhite,
    Color,
}

impl Display for BulbKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                BulbKind::DimmableWhite => "Dimmable White",
                BulbKind::TunableWhite => "Tunable White",
                BulbKind::Color => "Color",
            }
        )
    }
}

impl BulbKind {
    fn is_color(&self) -> bool {
        matches!(self, Self::Color)
    }
}

#[derive(Error, Debug)]
pub enum DeviceError {
    #[error("Client failed to initialize!\n{0}")]
    ClientInitError(#[source] io::Error),
    #[error("Failed to connect to device!\n{0}")]
    ConnectError(#[source] ConnectionError),
    #[error("Did not recognize module name: {0}!")]
    UnrecognizedModuleName(String),
    #[error("Failed to change the state of a device!\n{0}")]
    SetPilotError(#[source] ConnectionError),
    #[error("{0:?} devices do not support {1}!")]
    UnsupportedCommand(DeviceKind, String),
}
