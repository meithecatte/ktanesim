use enum_map::EnumMap;

pub struct Edgework {
    pub serial_number: SerialNumber,
    pub port_plates: Vec<PortPlate>,
    pub indicators: EnumMap<IndicatorCode, IndicatorState>,
    pub aa_battery_pairs: u8,
    pub dcell_batteries: u8,
}

pub struct SerialNumber(String);

impl SerialNumber {
    // O is replaced with E to avoid confusion with zeroes
    // Y is removed because some languages consider it a vowel and some don't
    // conclusion: not the alphabet
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
}

bitflags! {
    pub struct PortType: u8 {
        const Serial    = 0b000001;
        const Parallel  = 0b000010;
        const DVI       = 0b000100;
        const PS2       = 0b001000;
        const RJ45      = 0b010000;
        const StereoRCA = 0b100000;
    }
}

/// The port plate widget.
pub struct PortPlate(PortType);

impl PortPlate {
    /// Ports are divided into groups. All ports on a port plate must be from the same group,
    /// because they won't fit otherwise.
    const PORT_GROUPS: [&'static [PortType]; 2] = [
        &[PortType::Serial, PortType::Parallel],
        &[
            PortType::DVI,
            PortType::PS2,
            PortType::RJ45,
            PortType::StereoRCA,
        ],
    ];

    /// Create a new PortPlate object, after making sure that all ports belong to a single
    /// port group.
    pub fn new(ports: PortType) -> Option<Self> {
        for &port_group in &Self::PORT_GROUPS {
            let port_group: PortType = port_group.iter().cloned().collect();

            if port_group.contains(ports) {
                return Some(PortPlate(ports));
            }
        }

        None
    }
}

#[derive(Copy, Clone, Debug, FromPrimitive, AsStaticStr, Enum)]
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
    fn check_enum_count() {
        use num_traits::FromPrimitive;
        assert!(IndicatorCode::from_u32(IndicatorCode::COUNT).is_none());
        assert!(IndicatorCode::from_u32(IndicatorCode::COUNT - 1).is_some());
    }

    #[test]
    fn port_plate_constructor() {
        assert!(PortPlate::new(PortType::Serial | PortType::Parallel).is_some());
        assert!(PortPlate::new(PortType::empty()).is_some());
        assert!(PortPlate::new(PortType::DVI).is_some());
        assert!(PortPlate::new(PortType::DVI | PortType::Serial).is_none());
    }

    #[test]
    fn serial_number_constructor() {
        for &wrong in [
            "SN1BB3X", // Too long
            "SN1BB",   // Too short
            "SN½BB3", // Unicode peskiness
            "Ab1cD2", "AB!CD2", // Character classes
            "SN1234", "ABCDEF", // Wrong patterns
            "OB1EC7", "YU1NO2", // Forbidden characters
        ].iter() {
            assert!(SerialNumber::new(wrong.to_string()).is_none());
        }

        // These are fine
        assert!(SerialNumber::new("AB1CD2".to_string()).is_some());
        assert!(SerialNumber::new("123AB4".to_string()).is_some());
    }
}
