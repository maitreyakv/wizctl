use derive_getters::Getters;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Clone, Debug, Getters)]
pub struct RGBCW {
    r: u8,
    g: u8,
    b: u8,
    c: u8,
    w: u8,
}

impl Display for RGBCW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{},{},{},{})",
            self.r(),
            self.g(),
            self.b(),
            self.c(),
            self.w()
        )
    }
}

impl FromStr for RGBCW {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<u8> = s
            .split(",")
            .map(|v| v.parse::<u8>())
            .collect::<Result<Vec<u8>, _>>()
            .map_err(|_| ColorError::ParseError(s.to_string()))?;

        Ok(Self {
            r: *values
                .first()
                .ok_or(ColorError::ParseError(s.to_string()))?,
            g: *values.get(1).ok_or(ColorError::ParseError(s.to_string()))?,
            b: *values.get(2).ok_or(ColorError::ParseError(s.to_string()))?,
            c: *values.get(3).ok_or(ColorError::ParseError(s.to_string()))?,
            w: *values.get(4).ok_or(ColorError::ParseError(s.to_string()))?,
        })
    }
}

#[derive(Debug, Error)]
pub enum ColorError {
    #[error("Could not parse RGBCW values from \"{0}\"!")]
    ParseError(String),
}
