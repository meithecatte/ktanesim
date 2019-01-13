use enum_map::EnumMap;
use enumflags::BitFlags;
use std::fmt;

/// Represents the set of widgets on the edges of a bomb.
pub struct Edgework {
    pub serial_number: SerialNumber,
    pub port_plates: Vec<PortPlate>,
    pub indicators: EnumMap<IndicatorCode, IndicatorState>,
    pub aa_battery_pairs: u8,
    pub dcell_batteries: u8,
}

#[derive(Debug, Clone)]
pub struct SerialNumber(String);

impl AsRef<str> for SerialNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl SerialNumber {
    // O is replaced with E to avoid confusion with zeroes
    // Y is removed because some languages consider it a vowel and some don't
    // conclusion: not quite the alphabet
    pub const CHARSET: &'static str = "ABCDEFGHIJKLMNEPQRSTUVWXZ0123456789";

    fn is_valid_character(c: char) -> bool {
        Self::is_valid_letter(c) || Self::is_valid_digit(c)
    }

    fn is_valid_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    fn is_valid_letter(c: char) -> bool {
        c.is_ascii_uppercase() && c != 'O' && c != 'Y'
    }

    /// Create a new SerialNumber instance, after making sure the parameter is a valid bomb
    /// serial number.
    pub fn new(serial: String) -> Option<Self> {
        const PATTERN: [fn(char) -> bool; 6] = [
            SerialNumber::is_valid_character,
            SerialNumber::is_valid_character,
            SerialNumber::is_valid_digit,
            SerialNumber::is_valid_letter,
            SerialNumber::is_valid_letter,
            SerialNumber::is_valid_digit,
        ];

        if serial.len() != PATTERN.len() {
            return None;
        }

        for (c, func) in serial.chars().zip(PATTERN.iter()) {
            if !func(c) {
                return None;
            }
        }

        Some(SerialNumber(serial))
    }

    /// Returns the bytes that define the serial number.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_bytes()
    }

    /// Returns the trailing digit as an integer.
    pub fn last_digit(&self) -> u8 {
        self.as_bytes()[5] - b'0'
    }
}

/// A bitfield that represents the port types that can be present on a bomb.
#[derive(EnumFlags, Copy, Clone, Debug, PartialEq, EnumIter)]
#[repr(u8)]
pub enum PortType {
    Serial = 0b000001,
    Parallel = 0b000010,
    DVI = 0b000100,
    PS2 = 0b001000,
    RJ45 = 0b010000,
    StereoRCA = 0b100000,
}

impl fmt::Display for PortType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::PortType::*;
        match *self {
            Serial => write!(f, "serial"),
            Parallel => write!(f, "parallel"),
            DVI => write!(f, "DVI-D"),
            PS2 => write!(f, "PS/2"),
            RJ45 => write!(f, "RJ-45"),
            StereoRCA => write!(f, "Stereo RCA"),
        }
    }
}

/// The port plate widget.
#[derive(Debug, Copy, Clone)]
pub struct PortPlate(BitFlags<PortType>);

lazy_static! {
    /// Ports are divided into groups. All ports on a port plate must be from the same group,
    /// because they won't fit otherwise.
    static ref PORT_GROUPS: [BitFlags<PortType>; 2] = [
        PortType::Serial | PortType::Parallel,
        PortType::DVI | PortType::PS2 | PortType::RJ45 | PortType::StereoRCA,
    ];
}

impl PortPlate {
    /// Create a new PortPlate object, after making sure that all ports belong to a single
    /// port group.
    pub fn new(ports: BitFlags<PortType>) -> Option<Self> {
        if PORT_GROUPS.iter().any(|group| group.contains(ports)) {
            Some(PortPlate(ports))
        } else {
            None
        }
    }

    /// Returns true if and only if there are no ports on this port plate
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if and only if `port` is present on this port plate
    pub fn has(&self, port: PortType) -> bool {
        self.0.contains(port)
    }
}

impl From<PortPlate> for BitFlags<PortType> {
    fn from(plate: PortPlate) -> BitFlags<PortType> {
        plate.0
    }
}

#[derive(Debug, Copy, Clone, FromPrimitive, IntoStaticStr, EnumIter, Enum, PartialEq)]
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

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum IndicatorState {
    NotPresent,
    Unlit,
    Lit,
}

impl Default for IndicatorState {
    fn default() -> Self {
        IndicatorState::NotPresent
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum EdgeworkCondition {
    SerialStartsWithLetter,
    SerialOdd,
    HasEmptyPortPlate,
    PortPresent(PortType),
}

impl fmt::Display for EdgeworkCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::EdgeworkCondition::*;
        match *self {
            SerialStartsWithLetter => write!(f, "the serial number starts with a letter"),
            SerialOdd => write!(f, "the last digit of the serial number is odd"),
            HasEmptyPortPlate => write!(f, "there is an empty port plate present on the bomb"),
            PortPresent(port) => {
                let article = if port == PortType::RJ45 { "an" } else { "a" };
                write!(f, "there is {} {} port present on the bomb", article, port)
            }
        }
    }
}

impl EdgeworkCondition {
    /// Returns true if `edgework` satisfies the condition
    pub fn evaluate(&self, edgework: &Edgework) -> bool {
        use self::EdgeworkCondition::*;
        match *self {
            SerialStartsWithLetter => edgework.serial_number.as_bytes()[0].is_ascii_uppercase(),
            SerialOdd => edgework.serial_number.last_digit() % 2 == 1,
            HasEmptyPortPlate => edgework
                .port_plates
                .iter()
                .cloned()
                .any(|plate| plate.is_empty()),
            PortPresent(port) => edgework
                .port_plates
                .iter()
                .cloned()
                .any(|plate| plate.has(port)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_enum_count() {
        use num_traits::FromPrimitive;
        assert!(IndicatorCode::from_u32(IndicatorCode::COUNT).is_none());
        assert!(IndicatorCode::from_u32(IndicatorCode::COUNT - 1).is_some());
    }

    #[test]
    fn port_plate_constructor() {
        use super::PortType::*;
        assert!(PortPlate::new(Serial | Parallel).is_some());
        assert!(PortPlate::new(BitFlags::<PortType>::empty()).is_some());
        assert!(PortPlate::new(DVI.into()).is_some());
        assert!(PortPlate::new(DVI | Serial).is_none());
    }

    #[test]
    fn serial_number_constructor() {
        for &wrong in &[
            "SN1BB3X", // Too long
            "SN1BB",   // Too short
            "SN½BB3", "SN½B3", // Unicode peskiness
            "Ab1cD2", "AB!CD2", // Character classes
            "SN1234", "ABCDEF", // Wrong patterns
            "OB1EC7", "YU0NO0", // Forbidden characters
        ] {
            assert!(SerialNumber::new(wrong.into()).is_none());
        }

        for &fine in &["AB1CD2", "123AB4", "KT4NE8"] {
            assert!(SerialNumber::new(fine.into()).is_some());
        }
    }
}
