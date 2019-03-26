use enum_map::EnumMap;
use enumflags2::BitFlags;
use lazy_static::lazy_static;
use rand::prelude::*;
use strum_macros::{Display, EnumCount, EnumIter, EnumProperty, EnumString, IntoStaticStr};

/// Represents the set of widgets on the edges of a bomb.
#[derive(Debug, Clone, PartialEq)]
pub struct Edgework {
    pub serial_number: SerialNumber,
    pub port_plates: Vec<PortPlate>,
    pub indicators: EnumMap<IndicatorCode, IndicatorState>,
    pub aa_battery_pairs: u32,
    pub dcell_batteries: u32,
}

use std::str::FromStr;
impl FromStr for Edgework {
    type Err = EdgeworkParseError;
    /// Parse a Twitch Plays-style edgework string, for example:
    /// ```
    /// # use ktane_utils::edgework::*;
    /// # fn main() {
    /// #     use enum_map::enum_map;
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
        use self::EdgeworkParseError::*;
        use regex::Regex;

        lazy_static! {
            static ref REGEX: Regex =
                Regex::new(r"^(\d+)B\s+(\d+)H // (?:(.*) // )?([0-9A-Z]{6})$").unwrap();
        }

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
                                .map(str::parse::<PortType>)
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

use rand::distributions::Standard;
impl Distribution<Edgework> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Edgework {
        let serial_number = rng.gen();
        let mut dcell_batteries = 0;
        let mut aa_battery_pairs = 0;
        let mut port_plates = vec![];
        let mut indicators = EnumMap::new();
        use strum::IntoEnumIterator;
        let mut unused_indicators: Vec<_> = IndicatorCode::iter().collect();
        let mut unused_indicators = unused_indicators.partial_shuffle(rng, 5).0.iter();

        for _ in 0..5 {
            match rng.gen_range(0, 3) {
                0 => {
                    // battery widget
                    if rng.gen() {
                        dcell_batteries += 1;
                    } else {
                        aa_battery_pairs += 1;
                    }
                }
                // port plate widget
                1 => port_plates.push(random()),
                2 => {
                    // indicator widget
                    indicators[*unused_indicators.next().unwrap()] = if rng.gen_bool(0.6) {
                        IndicatorState::Lit
                    } else {
                        IndicatorState::Unlit
                    }
                }
                _ => unreachable!(),
            }
        }

        Edgework {
            serial_number,
            dcell_batteries,
            aa_battery_pairs,
            port_plates,
            indicators,
        }
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

    /// Checks whether a character can appear in a serial number
    pub fn is_valid_character(c: char) -> bool {
        Self::is_valid_letter(c) || Self::is_valid_digit(c)
    }

    /// Checks whether a character can appear in a serial number at a position that requires a
    /// digit.
    pub fn is_valid_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    /// Checks whether a character can appear in a serial number at a position that requires a
    /// letter.
    pub fn is_valid_letter(c: char) -> bool {
        c.is_ascii_uppercase() && c != 'O' && c != 'Y'
    }

    /// Returns a reference to the ASCII bytes that define the serial number.
    pub fn as_bytes(&self) -> &[u8] {
        &self.0.as_bytes()
    }

    /// Returns the 3rd character of the serial number, which is always a digit, as an integer.
    pub fn middle_digit(&self) -> u8 {
        self.as_bytes()[2] - b'0'
    }

    /// Returns the trailing digit as an integer.
    pub fn last_digit(&self) -> u8 {
        self.as_bytes()[5] - b'0'
    }
}

impl Distribution<SerialNumber> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> SerialNumber {
        let any = |rng: &mut R| *SerialNumber::CHARSET.as_bytes().choose(rng).unwrap() as char;
        let letter = |rng: &mut R| {
            *SerialNumber::CHARSET[..SerialNumber::CHARSET.len() - 10]
                .as_bytes()
                .choose(rng)
                .unwrap() as char
        };
        let digit = |rng: &mut R| (rng.gen_range(0, 10) + b'0') as char;

        let mut serial = String::with_capacity(6);
        serial.push(any(rng));
        serial.push(any(rng));
        serial.push(digit(rng));
        serial.push(letter(rng));
        serial.push(letter(rng));
        serial.push(digit(rng));
        SerialNumber(serial)
    }
}

use enumflags2_derive::EnumFlags;
/// A bitfield that represents the port types that can be present on a bomb.
#[derive(
    EnumFlags,
    Copy,
    Clone,
    Debug,
    Display,
    PartialEq,
    Eq,
    EnumString,
    EnumIter,
    EnumProperty,
    EnumCount,
    Hash,
)]
pub enum PortType {
    #[strum(to_string = "DVI-D", serialize = "DVI")]
    DVI = 0b00_0001,
    #[strum(to_string = "PS/2", serialize = "PS2")]
    PS2 = 0b00_0010,
    #[strum(to_string = "RJ-45", serialize = "RJ45", serialize = "RJ")]
    #[strum(props(article = "an"))]
    RJ45 = 0b00_0100,
    #[strum(to_string = "Stereo RCA", serialize = "StereoRCA", serialize = "RCA")]
    StereoRCA = 0b00_1000,
    #[strum(to_string = "parallel", serialize = "Parallel")]
    Parallel = 0b01_0000,
    #[strum(to_string = "serial", serialize = "Serial")]
    Serial = 0b10_0000,
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
    pub fn is_empty(self) -> bool {
        self.0.is_empty()
    }

    /// Returns true if and only if `port` is present on this port plate
    pub fn has(self, port: PortType) -> bool {
        self.0.contains(port)
    }
}

impl From<PortPlate> for BitFlags<PortType> {
    fn from(plate: PortPlate) -> BitFlags<PortType> {
        plate.0
    }
}

impl Distribution<PortPlate> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> PortPlate {
        let port_group = PORT_GROUPS.choose(rng).unwrap();

        let ports = port_group.iter().filter(|_| rng.gen()).collect();
        PortPlate(ports)
    }
}

use enum_map::Enum;
#[derive(
    Debug, Display, Copy, Clone, IntoStaticStr, EnumIter, EnumString, EnumCount, Enum, PartialEq, Eq,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn port_plate_validation() {
        use super::PortType::*;
        assert!(PortPlate::new(Serial | Parallel).is_some());
        assert!(PortPlate::new(BitFlags::empty()).is_some());
        assert!(PortPlate::new(DVI.into()).is_some());
        assert!(PortPlate::new(DVI | Serial).is_none());
    }

    #[test]
    fn serial_number_validation() {
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

        // parse results are tested in the doc comment
        for &test in &[
            "2B 1H // KT4NE8",
            "0B 0H // FRK // AA0AA0",
            "0B 0H // [Empty] // KT4NE8",
            "0B 0H // [RJ, RCA] // KT4NE8",
            "0B 0H // [serial] [Stereo RCA] // KT4NE8",
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
