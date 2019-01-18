use crate::edgework::{Edgework, EdgeworkCondition};
use crate::random::{RuleseedRandom, VANILLA_SEED};
use ordinal::Ordinal;
use smallvec::SmallVec;
use std::fmt;

/// Stores a full rule set for Wires.
pub struct WireRuleSet([WireRuleList; 4]);

impl WireRuleSet {
    pub const MIN_WIRES: usize = 3;
    pub const MAX_WIRES: usize = 6;

    /// Generates the rule set for a given `seed`.
    pub fn new(seed: u32) -> Self {
        if seed == VANILLA_SEED {
            WireRuleSet([
                WireRuleList {
                    rules: smallvec![
                        WireRule {
                            conditions: smallvec![WireQuery::ExactlyZeroOfColor(WireColor::Red)],
                            solution: WireSolution::Index(1),
                        },
                        WireRule {
                            conditions: smallvec![WireQuery::LastWireIs(WireColor::White)],
                            solution: WireSolution::Index(2),
                        },
                        WireRule {
                            conditions: smallvec![WireQuery::MoreThanOneOfColor(WireColor::Blue)],
                            solution: WireSolution::LastOfColor(WireColor::Blue),
                        },
                    ],
                    otherwise: WireSolution::Index(2),
                },
                WireRuleList {
                    rules: smallvec![
                        WireRule {
                            conditions: smallvec![
                                WireQuery::MoreThanOneOfColor(WireColor::Red),
                                WireQuery::EdgeworkCondition(EdgeworkCondition::SerialOdd),
                            ],
                            solution: WireSolution::LastOfColor(WireColor::Red),
                        },
                        WireRule {
                            conditions: smallvec![
                                WireQuery::LastWireIs(WireColor::Yellow),
                                WireQuery::ExactlyZeroOfColor(WireColor::Red),
                            ],
                            solution: WireSolution::Index(0),
                        },
                        WireRule {
                            conditions: smallvec![WireQuery::ExactlyOneOfColor(WireColor::Blue)],
                            solution: WireSolution::Index(0),
                        },
                        WireRule {
                            conditions: smallvec![WireQuery::MoreThanOneOfColor(WireColor::Yellow)],
                            solution: WireSolution::Index(3),
                        },
                    ],
                    otherwise: WireSolution::Index(1),
                },
                WireRuleList {
                    rules: smallvec![
                        WireRule {
                            conditions: smallvec![
                                WireQuery::LastWireIs(WireColor::Black),
                                WireQuery::EdgeworkCondition(EdgeworkCondition::SerialOdd),
                            ],
                            solution: WireSolution::Index(3),
                        },
                        WireRule {
                            conditions: smallvec![
                                WireQuery::ExactlyOneOfColor(WireColor::Red),
                                WireQuery::MoreThanOneOfColor(WireColor::Yellow),
                            ],
                            solution: WireSolution::Index(0),
                        },
                        WireRule {
                            conditions: smallvec![WireQuery::ExactlyZeroOfColor(WireColor::Black)],
                            solution: WireSolution::Index(1),
                        },
                    ],
                    otherwise: WireSolution::Index(0),
                },
                WireRuleList {
                    rules: smallvec![
                        WireRule {
                            conditions: smallvec![
                                WireQuery::ExactlyZeroOfColor(WireColor::Yellow),
                                WireQuery::EdgeworkCondition(EdgeworkCondition::SerialOdd),
                            ],
                            solution: WireSolution::Index(2),
                        },
                        WireRule {
                            conditions: smallvec![
                                WireQuery::ExactlyOneOfColor(WireColor::Yellow),
                                WireQuery::MoreThanOneOfColor(WireColor::White),
                            ],
                            solution: WireSolution::Index(3),
                        },
                        WireRule {
                            conditions: smallvec![WireQuery::ExactlyZeroOfColor(WireColor::Red)],
                            solution: WireSolution::Index(5),
                        },
                    ],
                    otherwise: WireSolution::Index(3),
                },
            ])
        } else {
            let _random = RuleseedRandom::new(seed);
            unimplemented!();
        }
    }

    /// If `wire_count` is a possible wire count, return a reference to the rules for the wire
    /// count.
    pub fn get(&self, wire_count: usize) -> Option<&WireRuleList> {
        if (Self::MIN_WIRES..=Self::MAX_WIRES).contains(&wire_count) {
            Some(&self.0[wire_count - Self::MIN_WIRES])
        } else {
            None
        }
    }

    /// If `wire_count` is a possible wire count, return a mutable reference to the rules for
    /// the wire count.
    pub fn get_mut(&mut self, wire_count: usize) -> Option<&mut WireRuleList> {
        if (Self::MIN_WIRES..=Self::MAX_WIRES).contains(&wire_count) {
            Some(&mut self.0[wire_count - Self::MIN_WIRES])
        } else {
            None
        }
    }

    /// Return the solution for a given module. If there are no rules for a given wire count,
    /// None is returned.
    pub fn evaluate(&self, edgework: &Edgework, wires: &[WireColor]) -> Option<WireSolution> {
        if let Some(rules) = self.get(wires.len()) {
            Some(rules.evaluate(edgework, wires))
        } else {
            None
        }
    }
}

/// Represents the rules for a particular wire count.
pub struct WireRuleList {
    pub rules: SmallVec<[WireRule; 4]>,
    /// The solution in case none of the rules applies.
    pub otherwise: WireSolution,
}

impl WireRuleList {
    pub fn evaluate(&self, edgework: &Edgework, wires: &[WireColor]) -> WireSolution {
        self.rules
            .iter()
            .filter(|rule| rule.evaluate(edgework, wires))
            .map(|rule| rule.solution)
            .next()
            .unwrap_or(self.otherwise)
    }
}

/// Represents a single sentence in the manual. If all `conditions` are met, the `solution`
/// applies (except earlier rules take precedence)
#[derive(Debug, Clone)]
pub struct WireRule {
    pub conditions: SmallVec<[WireQuery; 2]>,
    pub solution: WireSolution,
}

impl WireRule {
    pub fn evaluate(&self, edgework: &Edgework, wires: &[WireColor]) -> bool {
        self.conditions
            .iter()
            .all(|query| query.evaluate(edgework, wires))
    }
}

/// A single condition a particular rule might want to evaluate
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WireQuery {
    /// Represents a condition dependent only on the edgework
    EdgeworkCondition(EdgeworkCondition),
    /// If there is exactly one _color_ wire...
    ExactlyOneOfColor(WireColor),
    /// If there is more than one _color_ wire...
    MoreThanOneOfColor(WireColor),
    /// If there are no _color_ wires...
    ExactlyZeroOfColor(WireColor),
    /// If there last wire is _color_...
    LastWireIs(WireColor),
}

impl fmt::Display for WireQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::WireQuery::*;
        match *self {
            EdgeworkCondition(condition) => write!(f, "{}", condition),
            ExactlyOneOfColor(color) => write!(f, "there is exactly one {} wire", color),
            MoreThanOneOfColor(color) => write!(f, "there is more than one {} wire", color),
            ExactlyZeroOfColor(color) => write!(f, "there are no {} wires", color),
            LastWireIs(color) => write!(f, "the last wire is {}", color),
        }
    }
}

impl WireQuery {
    pub fn evaluate(&self, edgework: &Edgework, wires: &[WireColor]) -> bool {
        use self::WireQuery::*;
        let count_wires = |color| wires.iter().cloned().filter(|wire| *wire == color).count();
        match *self {
            EdgeworkCondition(condition) => condition.evaluate(edgework),
            LastWireIs(color) => *wires.last().unwrap() == color,
            ExactlyOneOfColor(color) => count_wires(color) == 1,
            MoreThanOneOfColor(color) => count_wires(color) > 1,
            ExactlyZeroOfColor(color) => count_wires(color) == 0,
        }
    }
}

/// The action the player should take to defuse a particular wire module
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum WireSolution {
    /// Cut the n-th wire. 0-indexed
    Index(u8),
    /// Cut the wire of the specified color. Only used when there is exactly one wire of the color.
    TheOneOfColor(WireColor),
    /// Cut the first wire of the specified color.
    FirstOfColor(WireColor),
    /// Cut the first wire of the specified color.
    LastOfColor(WireColor),
}

impl fmt::Display for WireSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::WireSolution::*;
        match *self {
            Index(n) => write!(f, "cut the {} wire", Ordinal(n + 1)),
            TheOneOfColor(color) => write!(f, "cut the {} wire", color),
            FirstOfColor(color) => write!(f, "cut the first {} wire", color),
            LastOfColor(color) => write!(f, "cut the last {} wire", color),
        }
    }
}

/// The colors a wire can have
#[derive(Debug, Copy, Clone, IntoStaticStr, EnumIter, PartialEq)]
pub enum WireColor {
    Black,
    Blue,
    Red,
    White,
    Yellow,
}

impl fmt::Display for WireColor {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", <&str>::from(*self).to_lowercase())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_wire_query() {
        for &(test, expected) in &[
            (
                WireQuery::EdgeworkCondition(EdgeworkCondition::SerialOdd),
                "the last digit of the serial number is odd",
            ),
            (
                WireQuery::ExactlyOneOfColor(WireColor::Yellow),
                "there is exactly one yellow wire",
            ),
        ] {
            assert_eq!(format!("{}", test), expected);
        }
    }

    #[test]
    fn display_wire_solution() {
        use super::WireSolution::*;
        for &(test, expected) in &[
            (Index(3), "cut the 4th wire"),
            (Index(1), "cut the 2nd wire"),
            (TheOneOfColor(WireColor::Red), "cut the red wire"),
        ] {
            assert_eq!(format!("{}", test), expected);
        }
    }

    #[test]
    fn display_wire_color() {
        assert_eq!(format!("{}", WireColor::Black), "black");
    }
}
