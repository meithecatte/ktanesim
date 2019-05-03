use crate::edgework::{IndicatorCode, PortType};
use enum_map::EnumMap;
use smallvec::SmallVec;
use strum_macros::{Display, EnumCount, EnumIter, IntoStaticStr};

/// The colors a button or LED strip can have.
#[derive(
    Debug, Display, Copy, Clone, IntoStaticStr, EnumCount, EnumIter, PartialEq, Eq, enum_map::Enum,
)]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    Red,
    Blue,
    Yellow,
    White,
}

/// The labels that can be shown on the button.
#[derive(Debug, Display, Copy, Clone, IntoStaticStr, EnumCount, EnumIter, PartialEq, Eq)]
pub enum Label {
    Press,
    Hold,
    Abort,
    Detonate,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSet {
    main_rules: SmallVec<[Rule; 7]>,
    release_rules: EnumMap<Color, ReleaseCondition>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    queries: SmallVec<[Query; 2]>,
    solution: Solution,
}

/// A single condition of a main rule.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Query {
    Color(Color),
    Label(Label),
    IndicatorLit(IndicatorCode),
    PortPresent(PortType),
    MoreBatteriesThan(u8),
    EmptyPortPlate,
    SerialStartsWithLetter,
    HasEmptyPortPlate,
}

/// The solution provided by a main rule.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Solution {
    Hold,
    Press,
    /// Only happens in non-vanilla rule sets.
    TapWhenSecondsMatch,
}

/// The condition that must be met when the button is released.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ReleaseCondition {
    /// Release when the countdown timer has a _digit_ in any position.
    DigitInAnyPosition(u8),
    /// Release when the rightmost seconds digit is _digit_.
    LastSecondsDigitIs(u8),
    /// Release when the two seconds digits add up to a number that meets the `SumCondition`.
    SecondsSum(SumCondition),
    /// Release when the number of seconds displayed is either prime or 0.
    SecondsPrimeOrZero,
    /// Release at any time.
    AnyTime,
}

/// Describes release conditions pertaining to the sum of the digits of the timer seconds.
///
/// Only happens in non-vanilla rule sets.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SumCondition {
    Five,
    Seven,
    ThreeOrThirteen,
    MultipleOfFour,
}
