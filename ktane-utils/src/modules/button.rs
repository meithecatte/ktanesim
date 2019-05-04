use crate::edgework::EdgeworkQuery;
use enum_map::EnumMap;
use joinery::prelude::*;
use smallvec::SmallVec;
use std::fmt;
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
#[strum(serialize_all="shouty_snake_case")]
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
pub struct MainRules {
    rules: SmallVec<[Rule; 6]>,
    otherwise: Solution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReleaseRules {
    rules: EnumMap<Color, ReleaseCondition>,
    order: SmallVec<[Color; COLOR_COUNT]>,
}

/// One line from the numbered list in the manual.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    queries: SmallVec<[Query; 2]>,
    solution: Solution,
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "If {}, {}.", self.queries.iter().join_with(" and "), self.solution)
    }
}

/// A single condition of a [`Rule`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Query {
    Color(Color),
    Label(Label),
    Edgework(EdgeworkQuery),
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Query::*;
        match *self {
            Color(color) => write!(f, "the button is {}", color),
            Label(label) => write!(f, r#"the button says "{}""#, label),
            Edgework(query) => query.fmt(f),
        }
    }
}

/// The solution provided by a main rule.
#[derive(Debug, Display, Copy, Clone, PartialEq, Eq)]
pub enum Solution {
    #[strum(serialize = r#"hold the button and refer to "Releasing a Held Button""#)]
    Hold,
    #[strum(serialize = "press and immediately release the button")]
    Press,
    /// Only happens in non-vanilla rule sets.
    #[strum(serialize = "press and immediately release when \
                         the two seconds digits on the timer match")]
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

impl fmt::Display for ReleaseCondition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ReleaseCondition::*;
        #[rustfmt::skip]
        match *self {
            DigitInAnyPosition(n) => {
                write!(f, "when the countdown timer has a {} in any position.", n)
            }
            LastSecondsDigitIs(n) => write!(f, "when the rightmost seconds digit is {}.", n),
            SecondsSum(cond) => write!(f, "when the two seconds digits add up to {}.", cond),
            SecondsPrimeOrZero => {
                write!(f, "when the the number of seconds displayed is either prime or 0.")
            }
            AnyTime => write!(f, "at any time"),
        }
    }
}

/// Describes release conditions pertaining to the sum of the digits of the timer seconds.
///
/// Only happens in non-vanilla rule sets.
#[derive(Debug, Display, Copy, Clone, PartialEq, Eq)]
pub enum SumCondition {
    #[strum(serialize = "5")]
    Five,
    #[strum(serialize = "7")]
    Seven,
    #[strum(serialize = "3 or 13")]
    ThreeOrThirteen,
    #[strum(serialize = "a multiple of 4")]
    MultipleOfFour,
}
