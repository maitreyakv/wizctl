use derive_getters::Getters;
use regex::Regex;
use std::{fmt::Display, str::FromStr};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ColorError {
    #[error("Could not parse RGBCW color component from {0}!")]
    CouldNotParseRGBCWComponent(String),
    #[error("Received only {0} components for RGBCW!")]
    InsufficientComponentsForRGBCW(usize),
}

#[derive(Clone, Default, Debug, Getters)]
pub struct RGBCW {
    r: u8,
    g: u8,
    b: u8,
    c: u8,
    w: u8,
}

impl RGBCW {
    pub fn white() -> Self {
        Self {
            r: 255,
            g: 255,
            b: 255,
            c: 255,
            w: 255,
        }
    }
}

impl Display for RGBCW {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "({},{},{},{},{})",
            self.r, self.g, self.b, self.c, self.w
        )
    }
}

impl FromStr for RGBCW {
    type Err = ColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"\d{1,3}").unwrap();
        let numbers = re
            .find_iter(s)
            .map(|m| {
                let s = m.as_str();
                s.parse::<u8>()
                    .map_err(|_| ColorError::CouldNotParseRGBCWComponent(m.as_str().to_string()))
            })
            .collect::<Result<Vec<u8>, ColorError>>()?;

        if numbers.len() < 5 {
            return Err(ColorError::InsufficientComponentsForRGBCW(numbers.len()));
        }

        Ok(RGBCW {
            r: *numbers.first().unwrap(),
            g: *numbers.get(1).unwrap(),
            b: *numbers.get(2).unwrap(),
            c: *numbers.get(3).unwrap(),
            w: *numbers.get(4).unwrap(),
        })
    }
}
