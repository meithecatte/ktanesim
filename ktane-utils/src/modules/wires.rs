use crate::edgework::Edgework;
use crate::random::{RuleseedRandom, VANILLA_SEED};
use smallvec::{smallvec, SmallVec};
use std::collections::HashMap;
use strum_macros::{Display, EnumCount, EnumIter, IntoStaticStr};

pub const MIN_WIRES: usize = 3;
pub const MAX_WIRES: usize = 6;

/// Stores a full rule set for Wires.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleSet([RuleList; 4]);

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
    ),+ or $otherwise_type:ident ($otherwise_arg:expr) ) => {
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
    /// Generates the rule set for a given `seed`.
    pub fn new(seed: u32) -> Self {
        if seed == VANILLA_SEED {
            use self::Color::*;

            RuleSet([
                rules! {
                    NotPresent(Red) => Index(1),
                    LastWireIs(White) => Index(2),
                    MoreThanOne(Blue) => LastOfColor(Blue)
                    or Index(2)
                },
                rules! {
                    MoreThanOne(Red), SerialOdd => LastOfColor(Red),
                    LastWireIs(Yellow), NotPresent(Red) => Index(0),
                    ExactlyOne(Blue) => Index(0),
                    MoreThanOne(Yellow) => Index(3)
                    or Index(1)
                },
                rules! {
                    LastWireIs(Black), SerialOdd => Index(3),
                    ExactlyOne(Red), MoreThanOne(Yellow) => Index(0),
                    NotPresent(Black) => Index(1)
                    or Index(0)
                },
                rules! {
                    NotPresent(Yellow), SerialOdd => Index(2),
                    ExactlyOne(Yellow), MoreThanOne(White) => Index(3),
                    NotPresent(Red) => Index(5)
                    or Index(3)
                },
            ])
        } else {
            let mut random = RuleseedRandom::new(seed);
            RuleSet(array_init::array_init(|index| {
                let wire_count = index + MIN_WIRES;
                let rule_count = Self::roll_rule_count(&mut random);
                let mut rules = smallvec![];
                let mut query_weights: HashMap<QueryWeightKey, f64> = HashMap::new();
                let mut solution_weights: HashMap<SolutionWeightKey, f64> = HashMap::new();

                while rules.len() < rule_count {
                    let rule = Self::generate_rule(
                        &mut random,
                        &mut query_weights,
                        &mut solution_weights,
                        wire_count,
                    );

                    if rule.is_valid() {
                        rules.push(rule);
                    }
                }

                // Put all the compound query rules in front
                rules.sort_by_key(|rule: &Rule| rule.queries.len().wrapping_neg());

                let mut solutions = Self::possible_solutions(&[], wire_count);
                let forbidden: SolutionWeightKey = rules.last().unwrap().solution.into();
                solutions.retain(|&mut solution| solution != forbidden);

                let otherwise = random
                    .choice(&solutions)
                    .unwrap()
                    .colorize(&mut random, &[]);

                RuleList { rules, otherwise }
            }))
        }
    }

    /// Return a random rule count for a specific wire count.
    fn roll_rule_count(random: &mut RuleseedRandom) -> usize {
        if random.next_double() < 0.6 {
            3
        } else {
            4
        }
    }

    /// Return a random query count for a specific rule.
    fn roll_compound(random: &mut RuleseedRandom) -> bool {
        random.next_double() >= 0.6
    }

    /// Generate a rule.
    fn generate_rule(
        random: &mut RuleseedRandom,
        query_weights: &mut HashMap<QueryWeightKey, f64>,
        solution_weights: &mut HashMap<SolutionWeightKey, f64>,
        wire_count: usize,
    ) -> Rule {
        let compound = Self::roll_compound(random);
        use strum::IntoEnumIterator;
        let mut colors_available_for_queries: SmallVec<[Color; COLOR_COUNT]> =
            Color::iter().collect();

        let available_queries: SmallVec<[_; WIREQUERYTYPE_COUNT]> =
            WireQueryType::iter().map(QueryWeightKey::Wire).collect();
        let main_query = Self::choose_query(
            random,
            &available_queries,
            query_weights,
            &mut colors_available_for_queries,
        );

        let auxiliary_query = if compound {
            let max_wires_involved = wire_count - main_query.wires_involved();
            use self::EdgeworkQuery::*;
            let available_queries: SmallVec<[_; QUERY_TYPE_COUNT]> =
                [SerialStartsWithLetter, SerialOdd]
                    .iter()
                    .cloned()
                    .map(QueryWeightKey::Edgework)
                    .chain(
                        WireQueryType::iter()
                            .filter(|query_type| query_type.wires_involved() < max_wires_involved)
                            .map(QueryWeightKey::Wire),
                    )
                    .chain(
                        PortType::iter()
                            .map(PortPresent)
                            .chain(std::iter::once(HasEmptyPortPlate))
                            .map(QueryWeightKey::Edgework),
                    )
                    .collect();

            Some(Self::choose_query(
                random,
                &available_queries,
                query_weights,
                &mut colors_available_for_queries,
            ))
        } else {
            None
        };

        let mut queries: SmallVec<[Query; 2]> = smallvec![main_query];
        queries.extend(auxiliary_query);

        let available_solutions = Self::possible_solutions(&queries, wire_count);

        let solution = *random.weighted_select(&available_solutions, solution_weights);
        *solution_weights.entry(solution).or_insert(1.0) *= 0.05;

        let solution_colors: SmallVec<[Color; 2]> = queries
            .iter()
            .cloned()
            .filter_map(Query::solution_colors)
            .collect();

        let solution = solution.colorize(random, &solution_colors);

        Rule { queries, solution }
    }

    fn choose_query(
        random: &mut RuleseedRandom,
        available_queries: &[QueryWeightKey],
        query_weights: &mut HashMap<QueryWeightKey, f64>,
        colors_available: &mut SmallVec<[Color; COLOR_COUNT]>,
    ) -> Query {
        let query_type = *random.weighted_select(&available_queries, query_weights);
        *query_weights.entry(query_type).or_insert(1.0) *= 0.1;
        query_type.colorize(random, colors_available)
    }

    fn possible_solutions(
        queries: &[Query],
        wire_count: usize,
    ) -> SmallVec<[SolutionWeightKey; 8]> {
        use self::SolutionWeightKey::*;
        let wire_count = wire_count as u8;
        let mut solutions = smallvec![Index(0), Index(1), Index(wire_count - 1)];

        solutions.extend((2..wire_count - 1).map(Index));
        solutions.extend(
            queries
                .iter()
                .cloned()
                .flat_map(Query::additional_solutions),
        );
        solutions
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for (rule_list, wire_count) in self.0.iter().zip(MIN_WIRES..) {
            write!(f, "{} wires:\n{}\n\n", wire_count, rule_list)?;
        }

        Ok(())
    }
}

use std::ops::Index;
impl Index<usize> for RuleSet {
    type Output = RuleList;

    fn index(&self, wire_count: usize) -> &RuleList {
        self.get(wire_count)
            .expect("index for RuleSet out of bounds")
    }
}

/// Represents the rules for a particular wire count.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuleList {
    pub rules: SmallVec<[Rule; 4]>,
    /// The solution in case none of the rules applies.
    pub otherwise: Solution,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut first_rule = true;

        for rule in self.rules.iter() {
            let beginning = if first_rule { "If" } else { "Otherwise, if" };

            write!(f, "{} ", beginning)?;

            let mut first_query = true;

            for &query in rule.queries.iter() {
                if !first_query {
                    write!(f, " and ")?;
                }

                write!(f, "{}", query)?;
                first_query = false;
            }

            writeln!(f, ", {}.", rule.solution)?;

            first_rule = false;
        }

        write!(f, "Otherwise, {}.", self.otherwise)
    }
}

/// Represents a single sentence in the manual. If all `queries` are met, the `solution` applies
/// (except earlier rules take precedence)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub queries: SmallVec<[Query; 2]>,
    pub solution: Solution,
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

        use self::Query::Wire;
        if let Wire(first) = self.queries[0] {
            if let Wire(second) = self.queries[1] {
                // The original algorithm considers many cases of rules that use the same color for
                // both queries. Because WireQueryType::colorize removes the chosen color from the
                // list of available colors, this situation can never actually arise. Hence, the
                // only thing we need to consider is two LastWireIs queries in the same rule, which
                // will obviously never be true at the same time.
                use self::WireQueryType::LastWireIs;
                if first.query_type == LastWireIs && second.query_type == LastWireIs {
                    return false;
                }
            }
        }

        true
    }
}

use crate::edgework::PortType;

/// A single condition of a rule
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Query {
    Edgework(EdgeworkQuery),
    Wire(WireQuery),
}

impl fmt::Display for Query {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Query::*;
        match self {
            Edgework(q) => q.fmt(f),
            Wire(q) => q.fmt(f),
        }
    }
}

impl Query {
    fn wires_involved(self) -> usize {
        use self::Query::*;
        match self {
            Edgework(_) => 0,
            Wire(query) => query.wires_involved(),
        }
    }

    fn additional_solutions(self) -> impl Iterator<Item = SolutionWeightKey> {
        use self::SolutionWeightKey::*;
        use self::WireQueryType::*;
        let solutions: &[_] = match self {
            Query::Wire(WireQuery { query_type, .. }) => match query_type {
                ExactlyOne => &[TheOneOfColor],
                MoreThanOne => &[FirstOfColor, LastOfColor],
                _ => &[],
            },
            _ => &[],
        };

        solutions.iter().cloned()
    }

    fn solution_colors(self) -> Option<Color> {
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum QueryWeightKey {
    Edgework(EdgeworkQuery),
    Wire(WireQueryType),
}

use crate::edgework::PORTTYPE_COUNT;
const QUERY_TYPE_COUNT: usize = WIREQUERYTYPE_COUNT + PORTTYPE_COUNT + 3;

impl From<Query> for QueryWeightKey {
    fn from(query: Query) -> Self {
        use self::QueryWeightKey::*;
        match query {
            Query::Edgework(q) => Edgework(q),
            Query::Wire(WireQuery { query_type, .. }) => Wire(query_type),
        }
    }
}

impl QueryWeightKey {
    fn colorize(
        self,
        random: &mut RuleseedRandom,
        colors_available: &mut SmallVec<[Color; COLOR_COUNT]>,
    ) -> Query {
        use self::QueryWeightKey::*;
        match self {
            Edgework(q) => Query::Edgework(q),
            Wire(query_type) => Query::Wire(query_type.colorize(random, colors_available)),
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

/// A condition pertaining to the edgework of a bomb
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum EdgeworkQuery {
    SerialStartsWithLetter,
    SerialOdd,
    HasEmptyPortPlate,
    PortPresent(PortType),
}

impl fmt::Display for EdgeworkQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::EdgeworkQuery::*;
        match self {
            SerialStartsWithLetter => write!(f, "the serial number starts with a letter"),
            SerialOdd => write!(f, "the last digit of the serial number is odd"),
            HasEmptyPortPlate => write!(f, "there is an empty port plate present on the bomb"),
            PortPresent(port) => {
                use strum::EnumProperty;
                let article = port.get_str("article").unwrap_or("a");
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

/// A condition pertaining to the colors of the wires on a module
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct WireQuery {
    query_type: WireQueryType,
    color: Color,
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

impl WireQueryType {
    fn wires_involved(self) -> usize {
        use self::WireQueryType::*;
        match self {
            MoreThanOne => 2,
            NotPresent => 0,
            _ => 1,
        }
    }

    fn colorize(
        self,
        random: &mut RuleseedRandom,
        colors_available: &mut SmallVec<[Color; COLOR_COUNT]>,
    ) -> WireQuery {
        let index = random.next_below(colors_available.len() as u32) as usize;
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

use std::fmt;
impl fmt::Display for WireQuery {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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

/// The action the player should take to defuse a particular wire module
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Solution {
    /// Cut the n-th wire. 0-indexed
    Index(u8),
    /// Cut the wire of the specified color. Only used when there is exactly one wire of the color.
    TheOneOfColor(Color),
    /// Cut the first wire of the specified color.
    FirstOfColor(Color),
    /// Cut the first wire of the specified color.
    LastOfColor(Color),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SolutionWeightKey {
    Index(u8),
    TheOneOfColor,
    FirstOfColor,
    LastOfColor,
}

impl From<Solution> for SolutionWeightKey {
    fn from(solution: Solution) -> SolutionWeightKey {
        match solution {
            Solution::Index(n) => SolutionWeightKey::Index(n),
            Solution::TheOneOfColor(_) => SolutionWeightKey::TheOneOfColor,
            Solution::FirstOfColor(_) => SolutionWeightKey::FirstOfColor,
            Solution::LastOfColor(_) => SolutionWeightKey::LastOfColor,
        }
    }
}

impl SolutionWeightKey {
    fn colorize(self, random: &mut RuleseedRandom, colors_available: &[Color]) -> Solution {
        use self::SolutionWeightKey::*;
        let color = random.choice(colors_available).cloned();
        match self {
            Index(n) => Solution::Index(n),
            TheOneOfColor => Solution::TheOneOfColor(color.unwrap()),
            FirstOfColor => Solution::FirstOfColor(color.unwrap()),
            LastOfColor => Solution::LastOfColor(color.unwrap()),
        }
    }
}

impl fmt::Display for Solution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
    /// Get the 0-indexed wire number for a given set of colors
    pub fn as_index(self, wires: &[Color]) -> Option<u8> {
        use self::Solution::*;
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
#[derive(Debug, Display, Copy, Clone, IntoStaticStr, EnumCount, EnumIter, PartialEq, Eq)]
#[strum(serialize_all = "snake_case")]
pub enum Color {
    Black,
    Blue,
    Red,
    White,
    Yellow,
}

#[cfg(test)]
mod tests {
    use super::Color::*;
    use super::*;

    #[test]
    fn display_query() {
        for &(test, expected) in &[
            (
                Query::Edgework(EdgeworkQuery::SerialOdd),
                "the last digit of the serial number is odd",
            ),
            (
                Query::Wire(WireQuery {
                    query_type: WireQueryType::ExactlyOne,
                    color: Yellow,
                }),
                "there is exactly one yellow wire",
            ),
        ] {
            assert_eq!(format!("{}", test), expected);
        }
    }

    #[test]
    fn display_solution() {
        use super::Solution::*;
        for &(test, expected) in &[
            (Index(3), "cut the 4th wire"),
            (Index(1), "cut the 2nd wire"),
            (TheOneOfColor(Red), "cut the red wire"),
        ] {
            assert_eq!(format!("{}", test), expected);
        }
    }

    #[test]
    fn display_color() {
        assert_eq!(format!("{}", Black), "black");
    }

    #[test]
    fn wire_query_evaluate() {
        use super::WireQueryType::*;

        const TESTS: &[(&[Color], WireQueryType, Color, bool)] = #[rustfmt::skip] &[
            (&[Red, Black, Blue], LastWireIs, Red, false),
            (&[Red, Black, Blue], LastWireIs, Blue, true),
            (&[Red, Black, Blue, Yellow], LastWireIs, Yellow, true),
            (&[Red, Black, Yellow, Blue], LastWireIs, Yellow, false),
            (&[Red, Black, Red, Blue], ExactlyOne, Red, false),
            (&[Red, Black, Blue, Yellow], ExactlyOne, Black, true),
            (&[Red, Black, Yellow, Blue, Yellow], MoreThanOne, Yellow, true),
            (&[Red, Black, Yellow, Blue, Red], MoreThanOne, Yellow, false),
            (&[Red, Black, Yellow, Black], NotPresent, Yellow, false),
            (&[Red, Black, Blue, Black], NotPresent, Yellow, true),
        ];

        for &(colors, query_type, color, expected) in TESTS {
            let query = WireQuery { query_type, color };
            assert_eq!(query.evaluate(colors), expected);
        }
    }

    #[test]
    fn edgework_query_evaluate() {
        use super::EdgeworkQuery::*;
        use super::PortType::*;

        #[rustfmt::skip]
        const TESTS: &[(&str, EdgeworkQuery, bool)] = &[
            ("0B 0H // KT4NE8", SerialStartsWithLetter, true),
            ("0B 0H // 123AB4", SerialStartsWithLetter, false),
            ("0B 0H // KT4NE8", SerialOdd, false),
            ("0B 0H // KT4NE7", SerialOdd, true),
            ("0B 0H // [Empty] // KT4NE8", HasEmptyPortPlate, true),
            ("0B 0H // [Serial] [Empty] // KT4NE8", HasEmptyPortPlate, true),
            ("0B 0H // KT4NE8", HasEmptyPortPlate, false),
            ("0B 0H // [Serial] [RCA] // KT4NE8", HasEmptyPortPlate, false),
            ("0B 0H // [Serial] // KT4NE8", PortPresent(Serial), true),
            ("0B 0H // [Serial, Parallel] // KT4NE8", PortPresent(Serial), true),
            ("0B 0H // [Serial, Parallel] // KT4NE8", PortPresent(Parallel), true),
            ("0B 0H // [Parallel] [Empty] // KT4NE8", PortPresent(Serial), false),
            ("0B 0H // [Parallel] [Serial] // KT4NE8", PortPresent(Serial), true),
            ("0B 0H // [Serial] [Parallel] // KT4NE8", PortPresent(Serial), true),
            ("0B 0H // KT4NE8", PortPresent(Serial), false),
        ];

        for &(edgework, query, expected) in TESTS {
            let edgework = edgework
                .parse::<Edgework>()
                .unwrap();
            assert_eq!(query.evaluate(&edgework), expected);
        }
    }

    #[test]
    fn generated_rules() {
        let rules = RuleSet::new(42);
        let expected = RuleSet([
            rules! {
                LastWireIs(Yellow), PortPresent(Serial) => Index(2),
                MoreThanOne(White), SerialStartsWithLetter => Index(1),
                ExactlyOne(Red) => Index(0),
                NotPresent(Black) => Index(2)
                or Index(0)
            },
            rules! {
                MoreThanOne(Red), HasEmptyPortPlate => Index(2),
                NotPresent(Yellow) => Index(0),
                LastWireIs(Yellow) => Index(1),
                MoreThanOne(Black) => FirstOfColor(Black)
                or Index(3)
            },
            rules! {
                MoreThanOne(Red), PortPresent(PS2) => Index(3),
                NotPresent(Black), SerialOdd => Index(2),
                LastWireIs(Red) => Index(1),
                ExactlyOne(White) => TheOneOfColor(White)
                or Index(1)
            },
            rules! {
                ExactlyOne(Blue), PortPresent(Parallel) => Index(4),
                NotPresent(Blue) => Index(1),
                LastWireIs(Black) => Index(2)
                or Index(1)
            },
        ]);

        assert_eq!(rules, expected);
    }

    #[test]
    fn vanilla_rules() {
        let rules = RuleSet::new(VANILLA_SEED);

        for &(edgework, colors, expected) in VANILLA_RULE_TESTS {
            let edgework = edgework.parse::<Edgework>().unwrap();
            let solution = rules.evaluate(&edgework, colors).as_index(colors).unwrap();
            assert_eq!(expected, solution);
        }
    }

    const VANILLA_RULE_TESTS: &[(&str, &[Color], u8)] = &[
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
