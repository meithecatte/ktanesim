use pretty_assertions::{assert_eq, assert_ne};
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
fn generated_rules() {
    let rules = RuleSet::new(42);
    let expected = RuleSet([
        rules! {
            LastWireIs(Yellow), PortPresent(Serial) => Index(2),
            MoreThanOne(White), SerialStartsWithLetter => Index(1),
            ExactlyOne(Red) => Index(0),
            NotPresent(Black) => Index(2)
            => Index(0)
        },
        rules! {
            MoreThanOne(Red), HasEmptyPortPlate => Index(2),
            NotPresent(Yellow) => Index(0),
            LastWireIs(Yellow) => Index(1),
            MoreThanOne(Black) => FirstOfColor(Black)
            => Index(3)
        },
        rules! {
            MoreThanOne(Red), PortPresent(PS2) => Index(3),
            NotPresent(Black), SerialOdd => Index(2),
            LastWireIs(Red) => Index(1),
            ExactlyOne(White) => TheOneOfColor(White)
            => Index(1)
        },
        rules! {
            ExactlyOne(Blue), PortPresent(Parallel) => Index(4),
            NotPresent(Blue) => Index(1),
            LastWireIs(Black) => Index(2)
            => Index(1)
        },
    ]);

    assert_eq!(rules, expected);
}

#[test]
fn vanilla_rules() {
    let rules = RuleSet::new(VANILLA_SEED);

    for &(edgework, colors, expected) in VANILLA_RULE_TESTS {
        let edgework = edgework.parse::<Edgework>().unwrap();
        let solution = rules.evaluate(&edgework, colors).as_index(colors);
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
