use super::{Color, ColorlessSolution, COLOR_COUNT};
use crate::edgework::{Edgework, EdgeworkQuery, PortType};
use crate::random::RuleseedRandom;
use smallvec::SmallVec;
use std::fmt;
use strum_macros::{EnumCount, EnumIter};

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

pub(super) const QUERY_TYPE_COUNT: usize =
    WIREQUERYTYPE_COUNT + crate::edgework::PORTTYPE_COUNT + 3;

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
        use self::WireQueryType::*;
        use super::ColorlessSolution::*;
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

    pub(super) fn primary_queries() -> SmallVec<[QueryType; WIREQUERYTYPE_COUNT]> {
        use strum::IntoEnumIterator;
        WireQueryType::iter().map(QueryType::Wire).collect()
    }

    pub(super) fn secondary_queries(uninvolved_wires: usize) -> SmallVec<[QueryType; QUERY_TYPE_COUNT]> {
        use super::EdgeworkQuery::*;
        use strum::IntoEnumIterator;

        // The serial number queries go first, then the wire colors, and ports at the end. This
        // order is necessary to generate the correct rules.
        [SerialStartsWithLetter, SerialOdd]
            .iter()
            .copied()
            .map(QueryType::Edgework)
            .chain(
                WireQueryType::iter()
                    // Can't use all uninvolved wires. Likely either an off-by-one in the
                    // original algorithm or a remaining wire is reserved for the solution.
                    .filter(|query_type| query_type.wires_involved() < uninvolved_wires)
                    .map(QueryType::Wire),
            )
            .chain(
                PortType::iter()
                    .map(PortPresent)
                    .chain(std::iter::once(HasEmptyPortPlate))
                    .map(QueryType::Edgework),
            )
            .collect()
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
        WireQuery {
            query_type: self,
            color: rng.choice_remove_small(colors_available),
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
