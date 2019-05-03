use crate::random::RuleseedRandom;
use super::Color;
use smallvec::{smallvec, SmallVec};
use std::fmt;

/// The action the player should take to defuse a particular wire module
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Solution {
    /// Cut the n-th wire. 0-indexed
    Index(u8),
    /// Cut the wire of the specified color. Only used when there is exactly one wire of the color.
    /// Well, that's what you'd hope for. It's actually possible to have more than one wire of the
    /// given color when this solution applies in some generated rule sets. For example, look at
    /// the first rule of 6 wires on rule seed 15. When this happens, cutting any of the wires of
    /// the given color is a correct solution. This is why `Solution::check` exists separately from
    /// `Solution::as_index`.
    TheOneOfColor(Color),
    /// Cut the first wire of the specified color.
    FirstOfColor(Color),
    /// Cut the first wire of the specified color.
    LastOfColor(Color),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub(super) enum ColorlessSolution {
    Index(u8),
    TheOneOfColor,
    FirstOfColor,
    LastOfColor,
}


impl From<Solution> for ColorlessSolution {
    fn from(solution: Solution) -> ColorlessSolution {
        match solution {
            Solution::Index(n) => ColorlessSolution::Index(n),
            Solution::TheOneOfColor(_) => ColorlessSolution::TheOneOfColor,
            Solution::FirstOfColor(_) => ColorlessSolution::FirstOfColor,
            Solution::LastOfColor(_) => ColorlessSolution::LastOfColor,
        }
    }
}

impl ColorlessSolution {
    pub(super) fn colorize(self, rng: &mut RuleseedRandom, colors_available: &[Color]) -> Solution {
        use self::ColorlessSolution::*;
        let color = rng.choice(colors_available).copied();
        match self {
            Index(n) => Solution::Index(n),
            TheOneOfColor => Solution::TheOneOfColor(color.unwrap()),
            FirstOfColor => Solution::FirstOfColor(color.unwrap()),
            LastOfColor => Solution::LastOfColor(color.unwrap()),
        }
    }

    pub(super) fn possible_solutions(
        queries: &[super::Query],
        wire_count: usize,
    ) -> SmallVec<[ColorlessSolution; 8]> {
        use self::ColorlessSolution::*;
        let wire_count = wire_count as u8;
        let mut solutions = smallvec![Index(0), Index(1), Index(wire_count - 1)];

        solutions.extend((2..wire_count - 1).map(Index));
        solutions.extend(
            queries
                .iter()
                .copied()
                .flat_map(super::Query::additional_solutions),
        );
        solutions
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use self::Solution::*;
        use ordinal::Ordinal;
        match self {
            Index(n) => write!(f, "cut the {} wire", Ordinal(n + 1)),
            TheOneOfColor(color) => write!(f, "cut the {} wire", color),
            FirstOfColor(color) => write!(f, "cut the first {} wire", color),
            LastOfColor(color) => write!(f, "cut the last {} wire", color),
        }
    }
}

impl Solution {
    /// Get one of the valid solutions as a 0-indexed wire number. If you are validating, use
    /// `check` instead, as some rulesets can create multiple valid solutions.
    pub fn as_index(self, wires: &[Color]) -> u8 {
        use self::Solution::*;
        match self {
            Index(n) => n,
            FirstOfColor(color) | TheOneOfColor(color) => {
                wires.iter().position(|&wire| wire == color).unwrap() as u8
            }
            LastOfColor(color) => wires.iter().rposition(|&wire| wire == color).unwrap() as u8,
        }
    }

    /// Checks if the right wire was cut. Use this if you're validating wire choice, since some
    /// rulesets have multiple valid solutions in some situations.
    pub fn check(self, wires: &[Color], index: u8) -> bool {
        use self::Solution::*;
        match self {
            TheOneOfColor(color) => wires[index as usize] == color,
            _ => self.as_index(wires) == index,
        }
    }
}
