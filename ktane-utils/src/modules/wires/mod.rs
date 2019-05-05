use crate::edgework::{Edgework, EdgeworkQuery};
use crate::random::{RuleseedRandom, VANILLA_SEED};
use joinery::prelude::*;
use rand::prelude::*;
use smallvec::{smallvec, SmallVec};
use defaultmap::DefaultHashMap;
use std::fmt;
use strum_macros::{Display, EnumCount, EnumIter, IntoStaticStr};

mod solution;
use solution::ColorlessSolution;
pub use solution::Solution;

mod query;
use query::QueryType;
pub use query::{Query, WireQuery, WireQueryType};

pub const MIN_WIRES: usize = 3;
pub const MAX_WIRES: usize = 6;

/// The colors a wire can have
#[derive(Debug, Display, Copy, Clone, IntoStaticStr, EnumCount, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    Black,
    Blue,
    Red,
    White,
    Yellow,
}

/// Stores a full rule set for Wires.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSet([RuleList; MAX_WIRES - MIN_WIRES + 1]);

/// Represents the rules for a particular wire count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleList {
    pub rules: SmallVec<[Rule; 4]>,
    /// The solution in case none of the `rules` applies.
    pub otherwise: Solution,
}

/// Represents a single sentence in the manual. If all `queries` are met, the `solution` applies
/// (except earlier rules take precedence)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub queries: SmallVec<[Query; 2]>,
    pub solution: Solution,
}

/// Generates a wire module just like the game. Each wire slot is represented as an Option, which
/// is set to `Some` if a wire is present, and to `None` otherwise. Additionally, the number of
/// wires actually present is returned as the second element of the tuple.
pub fn generate<R: Rng + ?Sized>(rng: &mut R) -> ([Option<Color>; MAX_WIRES], u8) {
    // 6 wires have more chance to occur
    let mut wire_count = rng.gen_range(MIN_WIRES, 9);

    if wire_count > MAX_WIRES {
        wire_count = MAX_WIRES;
    }

    let mut positions: [u8; MAX_WIRES] = array_init::array_init_copy(|i| i as u8);
    let mut wires = [None; MAX_WIRES];

    for &position in positions.partial_shuffle(rng, wire_count).0.iter() {
        use strum::IntoEnumIterator;
        wires[position as usize] = Color::iter().choose(rng);
    }

    (wires, wire_count as u8)
}

impl RuleSet {
    /// Generates the rule set for a given `seed`.
    pub fn new(seed: u32) -> Self {
        if seed == VANILLA_SEED {
            // At the bottom of the file to reduce clutter
            RuleSet::vanilla()
        } else {
            let mut rng = RuleseedRandom::new(seed);
            RuleSet(array_init::array_init(|index| {
                let wire_count = index + MIN_WIRES;
                let rule_count = Self::roll_rule_count(&mut rng);
                let mut rules = smallvec![];
                let mut query_weights = DefaultHashMap::new(1.0);
                let mut solution_weights = DefaultHashMap::new(1.0);

                while rules.len() < rule_count {
                    let rule = Rule::generate(
                        &mut rng,
                        &mut query_weights,
                        &mut solution_weights,
                        wire_count,
                    );

                    if rule.is_valid() {
                        rules.push(rule);
                    }
                }

                // Put all the compound query rules in front
                rules.sort_by_key(|rule: &Rule| std::cmp::Reverse(rule.queries.len()));

                let mut solutions = ColorlessSolution::possible_solutions(&[], wire_count);
                let forbidden: ColorlessSolution = rules.last().unwrap().solution.into();
                solutions.retain(|&mut solution| solution != forbidden);

                let otherwise = rng.choice(&solutions).unwrap().colorize(&mut rng, &[]);

                RuleList { rules, otherwise }
            }))
        }
    }

    /// Return a random rule count for a specific wire count.
    fn roll_rule_count(rng: &mut RuleseedRandom) -> usize {
        if rng.next_double() < 0.6 {
            3
        } else {
            4
        }
    }

    /// If `wire_count` is a possible wire count, return a reference to the rules for the wire
    /// count.
    pub fn get(&self, wire_count: usize) -> Option<&RuleList> {
        if (MIN_WIRES..=MAX_WIRES).contains(&wire_count) {
            Some(&self.0[wire_count - MIN_WIRES])
        } else {
            None
        }
    }

    /// If `wire_count` is a possible wire count, return a mutable reference to the rules for
    /// the wire count.
    pub fn get_mut(&mut self, wire_count: usize) -> Option<&mut RuleList> {
        if (MIN_WIRES..=MAX_WIRES).contains(&wire_count) {
            Some(&mut self.0[wire_count - MIN_WIRES])
        } else {
            None
        }
    }

    /// Return the solution for a given module. If there are no rules for a given wire count,
    /// None is returned.
    pub fn evaluate(&self, edgework: &Edgework, wires: &[Color]) -> Solution {
        self[wires.len()].evaluate(edgework, wires)
    }
}

impl fmt::Display for RuleSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for (rule_list, wire_count) in self.0.iter().zip(MIN_WIRES..) {
            write!(f, "{} wires:\n{}\n\n", wire_count, rule_list)?;
        }

        Ok(())
    }
}

impl std::ops::Index<usize> for RuleSet {
    type Output = RuleList;

    fn index(&self, wire_count: usize) -> &RuleList {
        self.get(wire_count)
            .expect("index for RuleSet out of bounds")
    }
}

impl RuleList {
    pub fn evaluate(&self, edgework: &Edgework, wires: &[Color]) -> Solution {
        self.rules
            .iter()
            .filter(|rule| rule.evaluate(edgework, wires))
            .map(|rule| rule.solution)
            .next()
            .unwrap_or(self.otherwise)
    }
}

impl fmt::Display for RuleList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first_rule = true;

        for rule in self.rules.iter() {
            let beginning = if first_rule { "If" } else { "Otherwise, if" };

            writeln!(
                f,
                "{} {}, {}.",
                beginning,
                rule.queries.iter().join_with(" and "),
                rule.solution,
            )?;

            first_rule = false;
        }

        write!(f, "Otherwise, {}.", self.otherwise)
    }
}

impl Rule {
    pub fn evaluate(&self, edgework: &Edgework, wires: &[Color]) -> bool {
        self.queries
            .iter()
            .all(|query| query.evaluate(edgework, wires))
    }

    fn is_valid(&self) -> bool {
        // A single query can never be redundant.
        if self.queries.len() == 1 {
            return true;
        }

        // The original algorithm considers many cases of rules that use the same color for
        // both queries. Because WireQueryType::colorize removes the chosen color from the
        // list of available colors, this situation can never actually arise. Hence, the
        // only thing we need to consider is two LastWireIs queries in the same rule, which
        // will obviously never be true at the same time.
        !self.queries.iter().all(|query| {
            if let self::Query::Wire(wire) = query {
                wire.query_type == self::WireQueryType::LastWireIs
            } else {
                false
            }
        })
    }

    /// Generate a rule.
    fn generate(
        rng: &mut RuleseedRandom,
        query_weights: &mut DefaultHashMap<QueryType, f64>,
        solution_weights: &mut DefaultHashMap<ColorlessSolution, f64>,
        wire_count: usize,
    ) -> Rule {
        let compound = rng.next_double() >= 0.6;

        use strum::IntoEnumIterator;
        let mut colors_available_for_queries: SmallVec<[Color; COLOR_COUNT]> =
            Color::iter().collect();

        let main_query = Self::choose_query(
            rng,
            &QueryType::primary_queries(),
            query_weights,
            &mut colors_available_for_queries,
        );

        let auxiliary_query = if compound {
            let uninvolved_wires = wire_count - main_query.wires_involved();

            Some(Self::choose_query(
                rng,
                &QueryType::secondary_queries(uninvolved_wires),
                query_weights,
                &mut colors_available_for_queries,
            ))
        } else {
            None
        };

        let mut queries: SmallVec<[Query; 2]> = smallvec![main_query];
        queries.extend(auxiliary_query);

        let available_solutions = ColorlessSolution::possible_solutions(&queries, wire_count);

        let solution = *rng.weighted_select(&available_solutions, solution_weights);
        solution_weights[solution] *= 0.05;

        let solution_colors: SmallVec<[Color; 2]> = queries
            .iter()
            .copied()
            .filter_map(Query::solution_color)
            .collect();

        let solution = solution.colorize(rng, &solution_colors);

        Rule { queries, solution }
    }

    fn choose_query(
        rng: &mut RuleseedRandom,
        available_queries: &[QueryType],
        query_weights: &mut DefaultHashMap<QueryType, f64>,
        colors_available: &mut SmallVec<[Color; COLOR_COUNT]>,
    ) -> Query {
        let query_type = *rng.weighted_select(&available_queries, query_weights);
        query_weights[query_type] *= 0.1;
        query_type.colorize(rng, colors_available)
    }
}

macro_rules! cond {
    ( $type:ident ) => {
        Query::Edgework(EdgeworkQuery::$type)
    };
    ( PortPresent ($port:ident) ) => {
        Query::Edgework(EdgeworkQuery::PortPresent(PortType::$port))
    };
    ( $type:ident ($color:ident) ) => {
        Query::Wire(WireQuery {
            query_type: WireQueryType::$type,
            color: Color::$color,
        })
    };
}

macro_rules! rules {
    ( $(
        $( $condition_type:tt $( ( $condition_arg:tt ) )? ),+ =>
        $solution_type:ident ($solution_arg:expr)
    ),+ => $otherwise_type:ident ($otherwise_arg:expr) ) => {
        RuleList {
            rules: smallvec![
                $(
                    Rule {
                        queries: smallvec![
                            $(
                                cond!($condition_type $(($condition_arg))?)
                            ),+
                        ],
                        solution: Solution::$solution_type($solution_arg),
                    }
                ),+
            ],
            otherwise: Solution::$otherwise_type($otherwise_arg),
        }
    };
}

impl RuleSet {
    // clippy does not like these macros...
    #[allow(clippy::cognitive_complexity)]
    fn vanilla() -> Self {
        use self::Color::*;

        RuleSet([
            rules! {
                NotPresent(Red) => Index(1),
                LastWireIs(White) => Index(2),
                MoreThanOne(Blue) => LastOfColor(Blue)
                => Index(2)
            },
            rules! {
                MoreThanOne(Red), SerialOdd => LastOfColor(Red),
                LastWireIs(Yellow), NotPresent(Red) => Index(0),
                ExactlyOne(Blue) => Index(0),
                MoreThanOne(Yellow) => Index(3)
                => Index(1)
            },
            rules! {
                LastWireIs(Black), SerialOdd => Index(3),
                ExactlyOne(Red), MoreThanOne(Yellow) => Index(0),
                NotPresent(Black) => Index(1)
                => Index(0)
            },
            rules! {
                NotPresent(Yellow), SerialOdd => Index(2),
                ExactlyOne(Yellow), MoreThanOne(White) => Index(3),
                NotPresent(Red) => Index(5)
                => Index(3)
            },
        ])
    }
}

#[cfg(test)]
use crate::edgework::PortType;

#[cfg(test)]
mod tests;
