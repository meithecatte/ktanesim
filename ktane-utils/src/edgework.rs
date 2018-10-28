use std::fmt;
use std::fmt::{Debug, Display};

pub struct Edgework {
    pub serial_number: SerialNumber,
    pub widgets: Vec<Widget>,
}

pub struct SerialNumber(pub String);

pub enum Widget {
    PortPlate,
    Indicator { code: IndicatorCode, lit: bool },
}

#[derive(Debug)]
pub enum PortType {
    Serial,
    Parallel,
    DVI,
    PS2,
    RJ45,
    StereoRCA,
}

#[derive(Debug, FromPrimitive)]
pub enum IndicatorCode {
    SND,
    CLR,
    CAR,
    IND,
    FRQ,
    SIG,
    NSA,
    MSA,
    TRN,
    BOB,
    FRK,
}

impl IndicatorCode {
    pub const COUNT: u32 = IndicatorCode::FRK as u32 + 1;
}

impl Display for IndicatorCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

pub struct Indicator {
    pub code: IndicatorCode,
    pub lit: bool,
}

use num_traits::FromPrimitive;
pub fn testing(x: u32) -> IndicatorCode {
    IndicatorCode::from_u32(x).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_traits::FromPrimitive;

    #[test]
    fn check_count() {
        assert!(IndicatorCode::from_u32(IndicatorCode::COUNT).is_none());
    }
}
