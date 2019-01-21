use crate::edgework::{Edgework, EdgeworkCondition};
use crate::random::VANILLA_SEED;
use smallvec::{smallvec, SmallVec};
use strum_macros::{Display, EnumIter, IntoStaticStr};

/// Stores a full rule set for Wires.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WireRuleSet([WireRuleList; 4]);

impl WireRuleSet {
    pub const MIN_WIRES: usize = 3;
    pub const MAX_WIRES: usize = 6;

    /// Generates the rule set for a given `seed`.
    pub fn new(seed: u32) -> Self {
        use crate::random::RuleseedRandom;

        if seed == VANILLA_SEED {
            // large dataset at the end of the file
            Self::vanilla_ruleset()
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
    pub fn evaluate(&self, edgework: &Edgework, wires: &[WireColor]) -> WireSolution {
        self[wires.len()].evaluate(edgework, wires)
    }
}

use std::ops::Index;
impl Index<usize> for WireRuleSet {
    type Output = WireRuleList;

    fn index(&self, wire_count: usize) -> &WireRuleList {
        self.get(wire_count)
            .expect("index for WireRuleSet out of bounds")
    }
}

/// Represents the rules for a particular wire count.
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum WireQuery {
    /// Represents a condition dependent only on the edgework
    EdgeworkFulfills(EdgeworkCondition),
    /// If there is exactly one _color_ wire...
    ExactlyOneOfColor(WireColor),
    /// If there is more than one _color_ wire...
    MoreThanOneOfColor(WireColor),
    /// If there are no _color_ wires...
    ExactlyZeroOfColor(WireColor),
    /// If there last wire is _color_...
    LastWireIs(WireColor),
}

use std::fmt;
impl fmt::Display for WireQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::WireQuery::*;
        match *self {
            EdgeworkFulfills(condition) => write!(f, "{}", condition),
            ExactlyOneOfColor(color) => write!(f, "there is exactly one {} wire", color),
            MoreThanOneOfColor(color) => write!(f, "there is more than one {} wire", color),
            ExactlyZeroOfColor(color) => write!(f, "there are no {} wires", color),
            LastWireIs(color) => write!(f, "the last wire is {}", color),
        }
    }
}

impl WireQuery {
    pub fn evaluate(self, edgework: &Edgework, wires: &[WireColor]) -> bool {
        use self::WireQuery::*;
        let count_wires = |color| wires.iter().cloned().filter(|wire| *wire == color).count();
        match self {
            EdgeworkFulfills(condition) => condition.evaluate(edgework),
            LastWireIs(color) => *wires.last().unwrap() == color,
            ExactlyOneOfColor(color) => count_wires(color) == 1,
            MoreThanOneOfColor(color) => count_wires(color) > 1,
            ExactlyZeroOfColor(color) => count_wires(color) == 0,
        }
    }
}

/// The action the player should take to defuse a particular wire module
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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
        use ordinal::Ordinal;
        match *self {
            Index(n) => write!(f, "cut the {} wire", Ordinal(n + 1)),
            TheOneOfColor(color) => write!(f, "cut the {} wire", color),
            FirstOfColor(color) => write!(f, "cut the first {} wire", color),
            LastOfColor(color) => write!(f, "cut the last {} wire", color),
        }
    }
}

impl WireSolution {
    /// Get the 0-indexed wire number for a given set of colors
    pub fn as_index(self, wires: &[WireColor]) -> Option<u8> {
        use self::WireSolution::*;
        match self {
            Index(n) => Some(n),
            FirstOfColor(color) | TheOneOfColor(color) => wires
                .iter()
                .position(|&wire| wire == color)
                .map(|x| x as u8),
            LastOfColor(color) => wires
                .iter()
                .rposition(|&wire| wire == color)
                .map(|x| x as u8),
        }
    }
}

/// The colors a wire can have
#[derive(Debug, Display, Copy, Clone, IntoStaticStr, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum WireColor {
    Black,
    Blue,
    Red,
    White,
    Yellow,
}

#[cfg(test)]
mod tests {
    use super::WireColor::*;
    use super::*;

    #[test]
    fn display_wire_query() {
        for &(test, expected) in &[
            (
                WireQuery::EdgeworkFulfills(EdgeworkCondition::SerialOdd),
                "the last digit of the serial number is odd",
            ),
            (
                WireQuery::ExactlyOneOfColor(Yellow),
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
            (TheOneOfColor(Red), "cut the red wire"),
        ] {
            assert_eq!(format!("{}", test), expected);
        }
    }

    #[test]
    fn display_wire_color() {
        assert_eq!(format!("{}", Black), "black");
    }

    #[test]
    fn wire_query_evaluate() {
        use super::EdgeworkCondition::*;
        use super::WireQuery::*;

        #[rustfmt::skip]
        const TESTS: &[(Option<&str>, &[WireColor], WireQuery, bool)] = &[
            (Some("0B 0H // 000AA0"), &[Red, Black, Blue], EdgeworkFulfills(SerialOdd), false),
            (Some("0B 0H // 000AA1"), &[Red, Black, Blue], EdgeworkFulfills(SerialOdd), true),
            (None, &[Red, Black, Blue], LastWireIs(Red), false),
            (None, &[Red, Black, Blue], LastWireIs(Blue), true),
            (None, &[Red, Black, Blue, Yellow], LastWireIs(Yellow), true),
            (None, &[Red, Black, Yellow, Blue], LastWireIs(Yellow), false),
            (None, &[Red, Black, Red, Blue], ExactlyOneOfColor(Red), false),
            (None, &[Red, Black, Blue, Yellow], ExactlyOneOfColor(Black), true),
            (None, &[Red, Black, Yellow, Blue, Yellow], MoreThanOneOfColor(Yellow), true),
            (None, &[Red, Black, Yellow, Blue, Red], MoreThanOneOfColor(Yellow), false),
            (None, &[Red, Black, Yellow, Black], ExactlyZeroOfColor(Yellow), false),
            (None, &[Red, Black, Blue, Black], ExactlyZeroOfColor(Yellow), true),
        ];

        for &(edgework, colors, query, expected) in TESTS {
            let edgework = edgework
                .unwrap_or("0B 0H // KT4NE8")
                .parse::<Edgework>()
                .unwrap();
            assert_eq!(query.evaluate(&edgework, colors), expected);
        }
    }

    #[test]
    fn vanilla_rules() {
        let rules = WireRuleSet::new(VANILLA_SEED);

        for &(edgework, colors, expected) in VANILLA_RULE_TESTS {
            let edgework = edgework.parse::<Edgework>().unwrap();
            let solution = rules.evaluate(&edgework, colors).as_index(colors).unwrap();
            assert_eq!(expected, solution);
        }
    }

    const VANILLA_RULE_TESTS: &[(&str, &[WireColor], u8)] = &[
        ("0B 0H // *SIG *BOB *CLR // [DVI, PS2, RJ45, StereoRCA] [Parallel] // GU4XA6",
         &[Red, Blue, Yellow], 2),
        ("0B 0H // *SIG *BOB *CLR // [DVI, PS2, RJ45, StereoRCA] [Parallel] // GU4XA6",
         &[White, Black, Yellow], 1),
        ("0B 0H // *SIG *BOB *CLR // [DVI, PS2, RJ45, StereoRCA] [Parallel] // GU4XA6",
         &[White, Red, Blue, Blue, Yellow], 1),
        ("0B 0H // *IND *MSA *NSA // [PS2, RJ45] [RJ45] // 9G2ZU9",
         &[Red, Red, White, Black, Black, Blue], 2),
        ("0B 0H // *IND *MSA *NSA // [PS2, RJ45] [RJ45] // 9G2ZU9",
         &[Yellow, Black, Red], 2),
        ("0B 0H // *IND *MSA *NSA // [PS2, RJ45] [RJ45] // 9G2ZU9",
         &[Blue, Black, White, White, Black, Blue], 2),
        ("3B 2H // FRK // [Parallel] [Parallel] // EC3WT3",
         &[White, Black, White], 1),
        ("3B 2H // FRK // [Parallel] [Parallel] // EC3WT3",
         &[Black, Yellow, Blue, Red, Black, Yellow], 3),
        ("3B 2H // FRK // [Parallel] [Parallel] // EC3WT3",
         &[White, Black, Red, Black, Blue, Black], 2),
        ("2B 2H // FRQ // [DVI, PS2, StereoRCA] [Serial] // LM8RR8",
         &[White, Black, Black], 1),
        ("5B 3H // [Serial] [Parallel] // G57WZ8",
         &[Blue, White, Black], 1),
        ("5B 3H // *SIG *BOB // LK8VF9",
         &[Black, Yellow, Yellow, Blue], 0),
        ("8B 4H // [Empty] // R23ED6",
         &[Red, White, Yellow, Red, Black], 0),
        ("1B 1H // TRN CAR // [Empty] [Parallel] // J00UG9",
         &[Yellow, Red, Blue, Black, Black, Yellow], 3),
        ("2B 1H // *SIG // [Serial, Parallel] [Empty] [DVI, StereoRCA] // RE3SE6",
         &[Blue, White, Blue], 1),
        ("0B 0H // *TRN *SND *FRQ *CLR // [Serial, Parallel] // 469AV3",
         &[Blue, Red, Blue], 2),
        ("3B 2H // CLR SND // [DVI, StereoRCA] // ME9PR9",
         &[Black, Yellow, Blue, Red, Black, White], 3),
        ("4B 3H // CLR TRN // RL3XC5",
         &[Blue, Black, White, Blue], 1),
        ("1B 1H // BOB *SIG // [Empty] [Parallel] // EH0II3",
         &[White, Red, White, Blue, Blue, Red], 2),
        ("2B 2H // *FRQ // [Serial, Parallel] [DVI, RJ45] // TS9FR8",
         &[Black, Blue, White], 1),
        ("1B 1H // NSA // [Serial] [Serial] [DVI, PS2] // 4A2SC5",
         &[White, Blue, Black, Red, Blue, White], 2),
        ("1B 1H // *BOB // [Serial, Parallel] [DVI, PS2, RJ45, StereoRCA] [Serial, Parallel] // DF3VJ8",
         &[Blue, Black, Red], 2),
        ("4B 2H // *CAR // [Parallel] [DVI, RJ45, StereoRCA] // VC2GK8",
         &[Blue, Red, Black, White, Black, White], 3),
        ("3B 2H // *FRK // [DVI, StereoRCA] [Parallel] // DC5LE2",
         &[Yellow, Yellow, White, Yellow, Black], 0),
        ("2B 2H // *IND *NSA // [DVI, PS2, RJ45, StereoRCA] // 394AG8",
         &[Black, Red, Red, Yellow, Yellow, Blue], 3),
        ("2B 1H // CAR SND IND // [Serial] // BX6RF2",
         &[Blue, Yellow, Black, Blue, Black], 0),
        ("1B 1H // FRK // [DVI] [Serial, Parallel] [DVI, PS2, RJ45, StereoRCA] // ZA3PU2",
         &[Blue, Blue, White, White, Yellow, Yellow], 5),
        ("6B 3H // [Serial, Parallel] [DVI, PS2, RJ45, StereoRCA] // 4W8ND9",
         &[Blue, Black, Black, Black], 0),
        ("3B 2H // *IND // [Serial] [DVI, PS2, RJ45] // 404BP2",
         &[Blue, Yellow, Yellow, Red, Black], 0),
        ("4B 2H // [RJ45, StereoRCA] [Parallel] [Serial, Parallel] // HQ9LP2",
         &[Black, Blue, White, Red, Black, White], 3),
        ("1B 1H // *SND *MSA *FRK // [DVI, StereoRCA] // RI1VB9",
         &[Red, Black, Black, Red], 3),
        ("2B 1H // *MSA // [Empty] [Parallel] [DVI, PS2, RJ45] // SJ6FF2",
         &[Blue, Yellow, Blue, Red, Black, White], 3),
        ("2B 1H // *BOB // [PS2] [Serial, Parallel] [DVI] // 5B3LP2",
         &[Red, Black, Red, Red, White, Red], 3),
        ("2B 1H // *NSA TRN MSA // [Empty] // Z76EN3",
         &[Blue, White, White, Black], 0),
        ("3B 2H // *SIG // [Serial] [DVI, RJ45, StereoRCA] // RE3XA5",
         &[Yellow, Black, White, Red, White, Red], 3),
        ("2B 1H // *BOB MSA // [PS2, StereoRCA] [Empty] // 1Z9PW1",
         &[Yellow, Black, White], 1),
        ("6B 3H // [Empty] [PS2] // J98ZX8",
         &[Yellow, Blue, Red], 2),
        ("5B 3H // *SIG // [Empty] // AE4CU1",
         &[White, Yellow, Yellow, Yellow, Red, Yellow], 3),
        ("0B 0H // *CAR // [DVI, PS2, RJ45] [PS2, StereoRCA] [RJ45] [Serial] // DP1CJ0",
         &[Black, Blue, White], 1),
        ("2B 2H // *SND *IND // [Parallel] // TN6JH5",
         &[Blue, Black, Red, Blue, White, Red], 2),
        ("1B 1H // *MSA *FRQ // [Empty] [PS2, RJ45] // JC6SV2",
         &[Red, Red, White, Blue], 0),
        ("0B 0H // *BOB *CAR SIG // [DVI, PS2, RJ45, StereoRCA] [Empty] // LU9IJ8",
         &[Red, Yellow, White, Red, Blue], 1),
        ("2B 1H // CLR *FRQ // [DVI] [RJ45] // T96EX7",
         &[Blue, Blue, Yellow, Yellow, Blue, White], 5),
        ("3B 2H // *MSA // [DVI, PS2, StereoRCA] [RJ45] // X31EK8",
         &[White, White, Red, Blue, Black, Blue], 3),
        ("3B 3H // [Serial] [Serial] // SH8XZ5",
         &[Red, Red, Yellow, Red, White], 1),
        ("2B 2H // [DVI, PS2, RJ45] [Serial, Parallel] [DVI, PS2, StereoRCA] // CB5CG7",
         &[White, Blue, Blue, Blue, Red, Blue], 2),
        ("7B 4H // [PS2] // UZ7JK9",
         &[White, White, Black, Black, Yellow, Red], 3),
        ("3B 2H // *FRQ *TRN // [DVI, RJ45, StereoRCA] // 170SB0",
         &[White, Yellow, Yellow], 1),
        ("1B 1H // *FRQ *CAR *MSA // [Serial, Parallel] // K90LT4",
         &[Yellow, Black, Black, Yellow, Yellow], 0),
        ("1B 1H // *FRQ *TRN // [DVI, StereoRCA] [Serial, Parallel] // 8C2NB2",
         &[Black, Blue, Blue, Blue, Blue, Red], 3),
        ("6B 4H // [DVI, RJ45] // PD2TE1",
         &[White, Black, Black, White, Blue], 0),
        ("5B 3H // [Parallel] [Serial, Parallel] // LS4HS0",
         &[White, Black, Blue, White, White, White], 5),
        ("2B 1H // CAR *IND FRQ // [PS2] // JJ5QE8",
         &[Yellow, Black, Red, Red, Blue, Black], 3),
        ("2B 2H // BOB // [Serial, Parallel] [PS2, StereoRCA] // BM0AT7",
         &[Blue, Red, Black, Black, Red, Blue], 2),
        ("1B 1H // CLR *TRN *SIG // [Serial, Parallel] // MP0CP0",
         &[Red, Red, Red, Yellow, Red, Yellow], 3),
        ("3B 3H // *MSA TRN // 3A2UI9",
         &[Yellow, Blue, Yellow, White, Blue, Red], 3),
        ("2B 2H // *MSA CAR *TRN // AF9FS2",
         &[Black, Yellow, White, Yellow, Red, White], 3),
        ("0B 0H // *FRK MSA // [DVI] [StereoRCA] [PS2, StereoRCA] // PH8MS8",
         &[Blue, Black, White, Blue, Black], 0),
        ("0B 0H // MSA *CAR // [DVI, StereoRCA] [PS2, RJ45, StereoRCA] [DVI, PS2, StereoRCA] // 285SX0",
         &[Red, Yellow, Red, Yellow, Yellow, Blue], 3),
    ];
}

impl WireRuleSet {
    fn vanilla_ruleset() -> Self {
        use self::WireColor::*;
        use self::WireQuery::*;
        use self::WireSolution::*;

        #[rustfmt::skip]
        WireRuleSet([
            WireRuleList {
                rules: smallvec![
                    WireRule {
                        conditions: smallvec![ExactlyZeroOfColor(Red)],
                        solution: Index(1),
                    },
                    WireRule {
                        conditions: smallvec![LastWireIs(White)],
                        solution: Index(2),
                    },
                    WireRule {
                        conditions: smallvec![MoreThanOneOfColor(Blue)],
                        solution: LastOfColor(Blue),
                    },
                ],
                otherwise: Index(2),
            },
            WireRuleList {
                rules: smallvec![
                    WireRule {
                        conditions: smallvec![
                            MoreThanOneOfColor(Red),
                            EdgeworkFulfills(EdgeworkCondition::SerialOdd),
                        ],
                        solution: LastOfColor(Red),
                    },
                    WireRule {
                        conditions: smallvec![
                            LastWireIs(Yellow),
                            ExactlyZeroOfColor(Red),
                        ],
                        solution: Index(0),
                    },
                    WireRule {
                        conditions: smallvec![ExactlyOneOfColor(Blue)],
                        solution: Index(0),
                    },
                    WireRule {
                        conditions: smallvec![MoreThanOneOfColor(Yellow)],
                        solution: Index(3),
                    },
                ],
                otherwise: Index(1),
            },
            WireRuleList {
                rules: smallvec![
                    WireRule {
                        conditions: smallvec![
                            LastWireIs(Black),
                            EdgeworkFulfills(EdgeworkCondition::SerialOdd),
                        ],
                        solution: Index(3),
                    },
                    WireRule {
                        conditions: smallvec![
                            ExactlyOneOfColor(Red),
                            MoreThanOneOfColor(Yellow),
                        ],
                        solution: Index(0),
                    },
                    WireRule {
                        conditions: smallvec![ExactlyZeroOfColor(Black)],
                        solution: Index(1),
                    },
                ],
                otherwise: Index(0),
            },
            WireRuleList {
                rules: smallvec![
                    WireRule {
                        conditions: smallvec![
                            ExactlyZeroOfColor(Yellow),
                            EdgeworkFulfills(EdgeworkCondition::SerialOdd),
                        ],
                        solution: Index(2),
                    },
                    WireRule {
                        conditions: smallvec![
                            ExactlyOneOfColor(Yellow),
                            MoreThanOneOfColor(White),
                        ],
                        solution: Index(3),
                    },
                    WireRule {
                        conditions: smallvec![ExactlyZeroOfColor(Red)],
                        solution: Index(5),
                    },
                ],
                otherwise: Index(3),
            },
        ])
    }
}
