use crate::edgework::{Edgework, PortType};
use crate::random::RuleseedRandom;
use smallvec::SmallVec;
use std::fmt;
use strum_macros::{EnumCount, EnumIter};
use super::{Color, COLOR_COUNT, ColorlessSolution};

/// A single condition of a rule
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Query {
    Edgework(EdgeworkQuery),
    Wire(WireQuery),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) enum QueryType {
    Edgework(EdgeworkQuery),
    Wire(WireQueryType),
}

pub(super) const QUERY_TYPE_COUNT: usize = WIREQUERYTYPE_COUNT + crate::edgework::PORTTYPE_COUNT + 3;

/// A condition pertaining to the edgework of a bomb
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EdgeworkQuery {
    SerialStartsWithLetter,
    SerialOdd,
    HasEmptyPortPlate,
    PortPresent(PortType),
}

/// A condition pertaining to the colors of the wires on a module
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WireQuery {
    pub query_type: WireQueryType,
    pub color: Color,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumCount, EnumIter)]
pub enum WireQueryType {
    /// If there is exactly one _color_ wire...
    ExactlyOne,
    /// If there are no _color_ wires...
    NotPresent,
    /// If there last wire is _color_...
    LastWireIs,
    /// If there is more than one _color_ wire...
    MoreThanOne,
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Query::*;
        match self {
            Edgework(q) => q.fmt(f),
            Wire(q) => q.fmt(f),
        }
    }
}

impl Query {
    pub(super) fn wires_involved(self) -> usize {
        use self::Query::*;
        match self {
            Edgework(_) => 0,
            Wire(query) => query.wires_involved(),
        }
    }

    pub(super) fn additional_solutions(self) -> impl Iterator<Item = ColorlessSolution> {
        use super::ColorlessSolution::*;
        use self::WireQueryType::*;
        let solutions: &[_] = match self {
            Query::Wire(WireQuery { query_type, .. }) => match query_type {
                ExactlyOne => &[TheOneOfColor],
                MoreThanOne => &[FirstOfColor, LastOfColor],
                _ => &[],
            },
            _ => &[],
        };

        solutions.iter().copied()
    }

    /// If the condition being true makes the presence of a wire of a given color certain, return
    /// that color.
    pub(super) fn solution_color(self) -> Option<Color> {
        use self::Query::*;
        use self::WireQueryType::*;
        match self {
            Wire(WireQuery { query_type, color }) => match query_type {
                ExactlyOne | MoreThanOne | LastWireIs => Some(color),
                _ => None,
            },
            _ => None,
        }
    }
}

impl From<Query> for QueryType {
    fn from(query: Query) -> Self {
        use self::QueryType::*;
        match query {
            Query::Edgework(q) => Edgework(q),
            Query::Wire(WireQuery { query_type, .. }) => Wire(query_type),
        }
    }
}

impl QueryType {
    pub(super) fn colorize(
        self,
        rng: &mut RuleseedRandom,
        colors_available: &mut SmallVec<[Color; COLOR_COUNT]>,
    ) -> Query {
        use self::QueryType::*;
        match self {
            Edgework(q) => Query::Edgework(q),
            Wire(query_type) => Query::Wire(query_type.colorize(rng, colors_available)),
        }
    }
}

impl Query {
    pub fn evaluate(self, edgework: &Edgework, wires: &[Color]) -> bool {
        use self::Query::*;
        match self {
            Edgework(query) => query.evaluate(edgework),
            Wire(query) => query.evaluate(wires),
        }
    }
}

impl fmt::Display for EdgeworkQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::EdgeworkQuery::*;
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

impl EdgeworkQuery {
    pub fn evaluate(self, edgework: &Edgework) -> bool {
        use self::EdgeworkQuery::*;
        match self {
            SerialStartsWithLetter => edgework.serial_number.as_bytes()[0].is_ascii_uppercase(),
            SerialOdd => edgework.serial_number.last_digit() % 2 == 1,
            HasEmptyPortPlate => edgework.port_plates.iter().any(|&plate| plate.is_empty()),
            PortPresent(port) => edgework.port_plates.iter().any(|&plate| plate.has(port)),
        }
    }
}

impl WireQueryType {
    pub(super) fn wires_involved(self) -> usize {
        use self::WireQueryType::*;
        match self {
            MoreThanOne => 2,
            NotPresent => 0,
            _ => 1,
        }
    }

    fn colorize(
        self,
        rng: &mut RuleseedRandom,
        colors_available: &mut SmallVec<[Color; COLOR_COUNT]>,
    ) -> WireQuery {
        let index = rng.next_below(colors_available.len() as u32) as usize;
        let color = colors_available.remove(index);

        WireQuery {
            query_type: self,
            color,
        }
    }
}

impl WireQuery {
    fn wires_involved(self) -> usize {
        self.query_type.wires_involved()
    }
}

impl fmt::Display for WireQuery {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::WireQueryType::*;
        match self.query_type {
            ExactlyOne => write!(f, "there is exactly one {} wire", self.color),
            MoreThanOne => write!(f, "there is more than one {} wire", self.color),
            NotPresent => write!(f, "there are no {} wires", self.color),
            LastWireIs => write!(f, "the last wire is {}", self.color),
        }
    }
}

impl WireQuery {
    pub fn evaluate(self, wires: &[Color]) -> bool {
        use self::WireQueryType::*;
        let count_wires = || wires.iter().filter(|&&wire| wire == self.color).count();
        match self.query_type {
            LastWireIs => *wires.last().unwrap() == self.color,
            ExactlyOne => count_wires() == 1,
            MoreThanOne => count_wires() > 1,
            NotPresent => count_wires() == 0,
        }
    }
}
