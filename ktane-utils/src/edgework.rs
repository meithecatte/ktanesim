use enum_map::EnumMap;
use enumflags::BitFlags;
use regex::Regex;
use std::fmt;
use std::str::FromStr;

/// Represents the set of widgets on the edges of a bomb.
#[derive(Debug, Clone, PartialEq)]
pub struct Edgework {
    pub serial_number: SerialNumber,
    pub port_plates: Vec<PortPlate>,
    pub indicators: EnumMap<IndicatorCode, IndicatorState>,
    pub aa_battery_pairs: u32,
    pub dcell_batteries: u32,
}

/// Errors that the FromStr implementation for Edgework can produce
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EdgeworkParseError {
    /// The string does not conform to the format
    FormatError,
    /// The combination of batteries and holders is impossible
    ImpossibleBatteries,
    /// The serial number is not valid
    MalformedSerial,
    /// A combination of ports on a port plate is impossible
    ImpossiblePortPlate,
    /// A section identified as port plates contains a port name that couldn't be recognized
    NotAPort,
    /// A section identified as indicators contains an unknown indicator
    NotAnIndicator,
}

impl FromStr for Edgework {
    type Err = EdgeworkParseError;
    /// Parse a Twitch Plays-style edgework string, for example:
    /// ```
    /// # use ktane_utils::edgework::*;
    /// # #[macro_use]
    /// # extern crate enum_map;
    /// # fn main() {
    /// #     assert_eq!(
    /// "2B 1H // *FRK CAR // [Serial] [DVI, RJ45] [Empty] // PG3NL1"
    /// #     .parse::<Edgework>().unwrap(), Edgework {
    /// #         serial_number: "PG3NL1".parse::<SerialNumber>().unwrap(),
    /// #         port_plates: vec![
    /// #             PortPlate::new(PortType::Serial.into()).unwrap(),
    /// #             PortPlate::new(PortType::DVI | PortType::RJ45).unwrap(),
    /// #             PortPlate::empty(),
    /// #         ],
    /// #         indicators: enum_map![
    /// #             IndicatorCode::FRK => IndicatorState::Lit,
    /// #             IndicatorCode::CAR => IndicatorState::Unlit,
    /// #             _ => IndicatorState::NotPresent,
    /// #         ],
    /// #         aa_battery_pairs: 1,
    /// #         dcell_batteries: 0,
    /// #     });
    /// # }
    /// ```
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^(\d+)B\s+(\d+)H // (?:(.*) // )?([0-9A-Z]{6})$").unwrap();
        }

        use self::EdgeworkParseError::*;

        let captures = REGEX.captures(input).ok_or(FormatError)?;

        // First, parse the battery section, which is always at the beginning
        let battery_count = captures[1]
            .parse::<u32>()
            .map_err(|_| ImpossibleBatteries)?;
        let holder_count = captures[2]
            .parse::<u32>()
            .map_err(|_| ImpossibleBatteries)?;
        let aa_battery_pairs = battery_count
            .checked_sub(holder_count)
            .ok_or(ImpossibleBatteries)?;
        let dcell_batteries = holder_count
            .checked_sub(aa_battery_pairs)
            .ok_or(ImpossibleBatteries)?;

        // Then goes the serial number, which is always at the end
        let serial_number = captures[4]
            .parse::<SerialNumber>()
            .map_err(|_| MalformedSerial)?;

        // What's left are the indicators and port plates. Note that these sections are
        // optional, and won't appear if no widget of a given type appears on the bomb.
        let mut indicators = EnumMap::new();
        let mut port_plates = vec![];
        if let Some(sections) = captures.get(3) {
            for section in sections.as_str().split(" // ") {
                if section.starts_with('[') && section.ends_with(']') {
                    // Port plate section
                    for port_plate in section[1..section.len() - 1].split("] [") {
                        if port_plate == "Empty" {
                            port_plates.push(PortPlate::empty());
                        } else {
                            let ports = port_plate
                                .split(", ")
                                .map(|port| port.parse::<PortType>())
                                .collect::<Result<BitFlags<_>, _>>()
                                .map_err(|_| NotAPort)?;
                            port_plates.push(PortPlate::new(ports).ok_or(ImpossiblePortPlate)?);
                        }
                    }
                } else {
                    // Indicator section
                    for indicator in section.split(' ') {
                        let (state, code) = if indicator.starts_with('*') {
                            (IndicatorState::Lit, &indicator[1..])
                        } else {
                            (IndicatorState::Unlit, indicator)
                        };

                        let code = code.parse::<IndicatorCode>().map_err(|_| NotAnIndicator)?;
                        indicators[code] = state;
                    }
                }
            }
        }

        Ok(Edgework {
            serial_number,
            indicators,
            port_plates,
            aa_battery_pairs,
            dcell_batteries,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SerialNumber(String);

impl AsRef<str> for SerialNumber {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl FromStr for SerialNumber {
    type Err = ();

    /// Create a new SerialNumber instance, after making sure the parameter is a valid bomb
    /// serial number.
    fn from_str(serial: &str) -> Result<Self, Self::Err> {
        const PATTERN: [fn(char) -> bool; 6] = [
            SerialNumber::is_valid_character,
            SerialNumber::is_valid_character,
            SerialNumber::is_valid_digit,
            SerialNumber::is_valid_letter,
            SerialNumber::is_valid_letter,
            SerialNumber::is_valid_digit,
        ];

        if serial.len() != PATTERN.len() {
            return Err(());
        }

        for (c, func) in serial.chars().zip(PATTERN.iter()) {
            if !func(c) {
                return Err(());
            }
        }

        Ok(SerialNumber(serial.to_string()))
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
#[derive(EnumFlags, Copy, Clone, Debug, PartialEq, Eq, EnumString, EnumIter)]
pub enum PortType {
    Serial    = 0b000001,
    Parallel  = 0b000010,
    DVI       = 0b000100,
    PS2       = 0b001000,
    #[strum(serialize = "RJ45", serialize = "RJ")]
    RJ45      = 0b010000,
    #[strum(serialize = "StereoRCA", serialize = "RCA")]
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
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct PortPlate(BitFlags<PortType>);

lazy_static! {
    /// Ports are divided into groups. All ports on a port plate must be from the same group,
    /// because they won't fit otherwise.
    ///
    /// This is a lazily initialized static variable. See [`lazy_static`] for more details.
    ///
    /// [`lazy_static`]: https://crates.io/crates/lazy_static
    pub static ref PORT_GROUPS: [BitFlags<PortType>; 2] = [
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

    /// Create an empty port plate
    pub fn empty() -> Self {
        PortPlate(BitFlags::empty())
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

#[derive(
    Debug,
    Display,
    Copy,
    Clone,
    FromPrimitive,
    IntoStaticStr,
    EnumIter,
    Enum,
    PartialEq,
    Eq,
    EnumString,
)]
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

/// A question that can be asked about the edgework by game rules
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
        assert!(PortPlate::new(BitFlags::empty()).is_some());
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
            assert!(wrong.parse::<SerialNumber>().is_err());
        }

        for &fine in &["AB1CD2", "123AB4", "KT4NE8"] {
            assert!(fine.parse::<SerialNumber>().is_ok());
        }
    }

    #[test]
    fn edgework_parser() {
        use super::EdgeworkParseError::*;

        // parse results are tested in the doccomment
        for &test in &[
            "2B 1H // KT4NE8",
            "0B 0H // FRK // AA0AA0",
            "0B 0H // [Empty] // KT4NE8",
            "0B 0H // [RJ, RCA] // KT4NE8",
        ] {
            if let Err(error) = test.parse::<Edgework>() {
                panic!("{} -> {:?}", test, error);
            }
        }

        for &(test, error) in &[
            ("5B 6H // KT4NE8", ImpossibleBatteries),
            ("12345678987654321B 0H // KT4NE8", ImpossibleBatteries),
            ("3B 1H // KT4NE8", ImpossibleBatteries),
            ("3B 2H // AB1234", MalformedSerial),
            ("3B 2H // -IND // 123AB4", NotAnIndicator),
            ("3B 2H // *WTF // KT4NE8", NotAnIndicator),
            ("0B 0H // WTF // KT4NE8", NotAnIndicator),
            ("3B 2H // [Airport] // KT4NE8", NotAPort),
        ] {
            assert_eq!(test.parse::<Edgework>(), Err(error));
        }
    }
}
