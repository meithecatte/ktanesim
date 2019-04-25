use std::collections::HashMap;
use std::hash::Hash;

const SEED_LEN: usize = 55;

/// The upper bound of the half-open range of values returned by [`RuleseedRandom::next_int`].
/// Also: the largest allowed rule seed.
pub const MAX_VALUE: u32 = <i32>::max_value() as u32;

/// The seed used for vanilla manuals
pub const VANILLA_SEED: u32 = 1;

/// A pseudorandom number generator that matches the one used by the Rule Seed Modifier mod.
pub struct RuleseedRandom {
    seed_array: [u32; SEED_LEN],
    next_index: usize,
}

impl RuleseedRandom {
    /// Subtracts two numbers with a specific, unusual wrapping behavior used by the RNG.
    fn diffwrap(lhs: u32, rhs: u32) -> u32 {
        match lhs.overflowing_sub(rhs) {
            (result, false) => result,
            (result, true) => result.wrapping_add(MAX_VALUE),
        }
    }

    /// Called to mangle the `seed_array` after using up all the random numbers in it.
    fn reseed(&mut self) {
        for i in 0..SEED_LEN {
            let j = (i + 31) % SEED_LEN;
            self.seed_array[i] = Self::diffwrap(self.seed_array[i], self.seed_array[j]);
        }
    }

    /// Creates a new RuleseedRandom instance using the provided seed.
    pub fn new(seed: u32) -> Self {
        let mut seed_array = [0; SEED_LEN];
        let mut last = 161_803_398 - seed;

        seed_array[SEED_LEN - 1] = last;

        let mut next = 1;

        for i in 1..SEED_LEN {
            let i = 21 * i % SEED_LEN - 1;
            seed_array[i] = next;
            let current = next;
            next = Self::diffwrap(last, next);
            last = current;
        }

        let mut random = Self {
            seed_array,
            next_index: 0,
        };

        for _ in 0..5 {
            random.reseed();
        }

        random
    }

    /// Generates a random `f64` in the half-open `[0; 1)` range. Please note that not all
    /// representable values in that range are possible results.
    pub fn next_double(&mut self) -> f64 {
        let result = self.seed_array[self.next_index];

        self.next_index += 1;

        if self.next_index >= SEED_LEN {
            self.reseed();
            self.next_index = 0;
        }

        // NOTE: dividing instead of multiplying by the reciprocal causes occasional off-by-ones.
        // That's also why next_int is implemented in terms of next_double and not the other way
        // around - it wouldn't match that way.
        f64::from(result) * (1. / f64::from(MAX_VALUE))
    }

    /// Generates a random 31-bit unsigned integer.
    pub fn next_int(&mut self) -> u32 {
        self.next_below(MAX_VALUE)
    }

    /// Generates a random integer with a value less than the `max_value` parameter.
    pub fn next_below(&mut self, max_value: u32) -> u32 {
        if max_value <= 1 {
            0
        } else {
            (self.next_double() * f64::from(max_value)) as u32
        }
    }

    /// Generates a random integer in the half-open `[lower_bound; upper_bound)` range.
    ///
    /// # Panics
    ///
    /// The method will panic if the range is empty.
    pub fn next(&mut self, lower_bound: u32, upper_bound: u32) -> u32 {
        assert!(
            lower_bound <= upper_bound,
            "RuleseedRandom::next: range is empty"
        );
        self.next_below(upper_bound - lower_bound) + lower_bound
    }

    /// Shuffles the provided `slice`. Equivalent to `.OrderBy(x => rnd.NextDouble())` in C#.
    /// Please consider using `shuffle_fisher_yates` instead.
    pub fn shuffle_by_sorting<T>(&mut self, slice: &mut [T]) {
        match slice.len() {
            0 => (),
            1 => {
                self.next_double();
            }
            _ => {
                use ordered_float::NotNan;
                slice.sort_by_cached_key(|_| NotNan::new(self.next_double()).unwrap());
            }
        };
    }

    /// Shuffles the provided `slice` using the Fisher-Yates algorithm. Recommended over
    /// `shuffle_by_sorting`.
    pub fn shuffle_fisher_yates<T>(&mut self, slice: &mut [T]) {
        for i in (1..slice.len()).rev() {
            let j = self.next_below((i + 1) as u32) as usize;
            slice.swap(i, j);
        }
    }

    /// Returns a randomly chosen element from the slice, or `None` if empty.
    pub fn choice<'s, T>(&mut self, slice: &'s [T]) -> Option<&'s T> {
        use std::convert::TryInto;
        let index = self.next_below(slice.len().try_into().expect("slice too large"));
        slice.get(index as usize)
    }

    /// Given a `Vec<T>` and a `HashMap<T, f64>`, perform a weighted random selection from the
    /// `Vec`, using the corresponding values in the `HashMap` as weights.
    pub fn weighted_select<'s, T>(&mut self, elements: &'s [T], weights: &HashMap<T, f64>) -> &'s T
    where
        T: Hash + Eq,
    {
        let total_weights: f64 = elements
            .iter()
            .map(|element| weights.get(element).copied().unwrap_or(1.0))
            .sum();
        let mut choice = self.next_double() * total_weights;

        for element in elements.iter() {
            let weight = weights.get(element).copied().unwrap_or(1.0);
            if choice < weight {
                return element;
            } else {
                choice -= weight;
            }
        }

        panic!("weighted_select tried to choose from zero elements");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_rng() -> RuleseedRandom {
        RuleseedRandom::new(123)
    }

    #[test]
    fn ints() {
        let mut random = get_rng();

        for &expected in EXPECTED_INTS.iter() {
            assert_eq!(random.next_int(), expected);
        }
    }

    #[test]
    fn doubles() {
        let mut random = get_rng();

        for &expected in EXPECTED_DOUBLES.iter() {
            assert_eq!(random.next_double(), expected);
        }
    }

    #[test]
    fn below() {
        let mut random = get_rng();

        for &expected in EXPECTED_BELOW.iter() {
            assert_eq!(random.next_below(100), expected);
        }
    }

    #[test]
    fn range() {
        let mut random = get_rng();

        for &expected in EXPECTED_RANGE.iter() {
            assert_eq!(random.next(10, 20), expected);
        }
    }

    #[test]
    fn shuffle_by_sort() {
        let mut random = get_rng();

        for &expected in EXPECTED_SHUFFLE_BY_SORTING.iter() {
            let mut test_vec: Vec<u8> = (0..expected.len() as u8).collect();
            random.shuffle_by_sorting(&mut test_vec);
            assert_eq!(test_vec, expected);
        }
    }

    #[test]
    fn shuffle_fisher_yates() {
        let mut random = get_rng();

        for &expected in EXPECTED_SHUFFLE_FISHER_YATES.iter() {
            let mut test_vec: Vec<u8> = (0..expected.len() as u8).collect();
            random.shuffle_fisher_yates(&mut test_vec);
            assert_eq!(test_vec, expected);
        }
    }

    #[test]
    #[should_panic]
    fn weighted_select_empty() {
        let mut random = get_rng();
        random.weighted_select::<u8>(&[], &std::collections::HashMap::new());
    }

    const EXPECTED_INTS: [u32; 96] = [
        0x69cb4d51, 0x21d9b60d, 0x2db970fa, 0x4ca3339c, 0x5fd6ca7f, 0x7572119b, 0x12143312,
        0x6cadbdf5, 0x60ca0aa6, 0x57ffbbdd, 0x5f03f75d, 0x06cb3358, 0x35fda2ae, 0x2f645fb8,
        0x3c561089, 0x1120f216, 0x5e879587, 0x1464dcdd, 0x438f0ba9, 0x4fb9a1f3, 0x64ccd5b9,
        0x58dfae1c, 0x0eba510d, 0x57cbca32, 0x44b5c4ff, 0x5f9929ff, 0x1621f56b, 0x4570fe9c,
        0x0c65c854, 0x3ea6dc28, 0x6eb4caf5, 0x02b55751, 0x3ea57613, 0x2f16d878, 0x6abdc702,
        0x796132da, 0x1e9ae1d7, 0x52c8b50f, 0x5656f9f3, 0x5b19ffc1, 0x1ae8722f, 0x706d4f4e,
        0x1477b0be, 0x1995073f, 0x708fa238, 0x7509314b, 0x21154f8b, 0x145df3a4, 0x5a3ee8d8,
        0x49f2d31a, 0x0c005266, 0x6f0eb3d1, 0x75e2f623, 0x7fb4beaf, 0x0f22f046, 0x6715f600,
        0x63343ff9, 0x7ea29881, 0x61e56c99, 0x667597a4, 0x56d72fc4, 0x3f4b7e02, 0x1656c402,
        0x05b00ae5, 0x3d1749ae, 0x6e96a80e, 0x72538299, 0x1c689b6f, 0x3ed4bd7f, 0x474cdf3d,
        0x700ba28a, 0x4a29a1e3, 0x3a25f404, 0x799c388e, 0x43b94f8d, 0x75be21e7, 0x62fcb7f8,
        0x0f05925d, 0x48a8d9ec, 0x5d9fceff, 0x7c64ea05, 0x177f5ce9, 0x638b9202, 0x25f030af,
        0x67cfac63, 0x2f694cf3, 0x6c5e934e, 0x38f56b2e, 0x71ff8ec9, 0x7c271ef3, 0x070db041,
        0x02324667, 0x13f3f790, 0x0f0a1ab6, 0x6b0e5d36, 0x50bed04b,
    ];

    #[rustfmt::skip]
    const EXPECTED_DOUBLES: [f64; 96] = [
        0.8265167855781116,  0.26445651672056714, 0.35722171997522084, 0.5987305178301086,
        0.7487424014828831,  0.9175436030689365,  0.14124143968394093, 0.8490521860537362,
        0.7561658205260363,  0.6874918777903039,  0.7423085410810581,  0.0530761890360509,
        0.4218028403920135,  0.3702506685490956,  0.471376483082481,   0.13381792145493343,
        0.7385126984391887,  0.1593280868415386,  0.527802903916595,   0.6228525580944738,
        0.7875010640302212,  0.6943261757000937,  0.11506093159087977, 0.6859066722383288,
        0.5367971661206321,  0.7468616961254094,  0.1729113362603408,  0.542510820805333,
        0.09685615640918545, 0.48946716286682856, 0.8648923588287515,  0.02115909057723316,
        0.4894244761622625,  0.3678846919759571,  0.8339165462338909,  0.9482787674983398,
        0.23910162934991608, 0.6467500997924945,  0.6745293083016431,  0.7117309280260145,
        0.21021868996798,    0.878335870280087,   0.159902661181941,   0.19986048303538023,
        0.8793833520633091,  0.9143430348086837,  0.2584628515217746,  0.15911717906553166,
        0.7050448473100759,  0.5777229194425619,  0.0937598227028548,  0.8676361822838132,
        0.9209888148684934,  0.9977033957828317,  0.11825374053709849, 0.8053576950008784,
        0.7750320405583047,  0.9893370279992637,  0.7648139715962177,  0.8004636339845432,
        0.6784419737190204,  0.49449133989144645, 0.17452287775209308, 0.04443489250002191,
        0.47727318782232386, 0.8639726708009712,  0.8931735278541099,  0.22194235735663323,
        0.49086731648578646, 0.5570334482737973,  0.8753550699331588,  0.579395519373657,
        0.45428323953146266, 0.9500799844740331,  0.529092735391619,   0.919864881746408,
        0.7733373608316003,  0.11735753580804799, 0.5676529317012303,  0.7314394711197537,
        0.9718296555671048,  0.18357430826107707, 0.7776968492091153,  0.29639252242464226,
        0.8110251891478082,  0.3704010189373051,  0.8466362128251401,  0.44498958366224056,
        0.8906115041536332,  0.9699438754329196,  0.05510523964422999, 0.01715927199328284,
        0.155882783306708,   0.11749586002784589, 0.8363758580928556,  0.630823170594323,
    ];

    const EXPECTED_BELOW: [u32; 200] = [
        82, 26, 35, 59, 74, 91, 14, 84, 75, 68, 74, 5, 42, 37, 47, 13, 73, 15, 52, 62, 78, 69, 11,
        68, 53, 74, 17, 54, 9, 48, 86, 2, 48, 36, 83, 94, 23, 64, 67, 71, 21, 87, 15, 19, 87, 91,
        25, 15, 70, 57, 9, 86, 92, 99, 11, 80, 77, 98, 76, 80, 67, 49, 17, 4, 47, 86, 89, 22, 49,
        55, 87, 57, 45, 95, 52, 91, 77, 11, 56, 73, 97, 18, 77, 29, 81, 37, 84, 44, 89, 96, 5, 1,
        15, 11, 83, 63, 42, 20, 67, 95, 14, 14, 59, 97, 60, 91, 8, 62, 18, 74, 95, 33, 9, 79, 74,
        66, 33, 5, 20, 84, 43, 68, 55, 53, 41, 73, 98, 48, 34, 61, 82, 14, 93, 81, 77, 64, 8, 98,
        55, 14, 3, 78, 23, 4, 53, 37, 46, 62, 70, 10, 64, 94, 86, 5, 12, 99, 21, 77, 20, 96, 82,
        10, 7, 3, 71, 16, 9, 5, 26, 37, 19, 71, 35, 10, 20, 49, 81, 49, 40, 42, 52, 21, 27, 38, 79,
        72, 7, 89, 10, 60, 54, 3, 71, 17, 95, 31, 43, 13, 84, 3,
    ];

    const EXPECTED_RANGE: [u32; 200] = [
        18, 12, 13, 15, 17, 19, 11, 18, 17, 16, 17, 10, 14, 13, 14, 11, 17, 11, 15, 16, 17, 16, 11,
        16, 15, 17, 11, 15, 10, 14, 18, 10, 14, 13, 18, 19, 12, 16, 16, 17, 12, 18, 11, 11, 18, 19,
        12, 11, 17, 15, 10, 18, 19, 19, 11, 18, 17, 19, 17, 18, 16, 14, 11, 10, 14, 18, 18, 12, 14,
        15, 18, 15, 14, 19, 15, 19, 17, 11, 15, 17, 19, 11, 17, 12, 18, 13, 18, 14, 18, 19, 10, 10,
        11, 11, 18, 16, 14, 12, 16, 19, 11, 11, 15, 19, 16, 19, 10, 16, 11, 17, 19, 13, 10, 17, 17,
        16, 13, 10, 12, 18, 14, 16, 15, 15, 14, 17, 19, 14, 13, 16, 18, 11, 19, 18, 17, 16, 10, 19,
        15, 11, 10, 17, 12, 10, 15, 13, 14, 16, 17, 11, 16, 19, 18, 10, 11, 19, 12, 17, 12, 19, 18,
        11, 10, 10, 17, 11, 10, 10, 12, 13, 11, 17, 13, 11, 12, 14, 18, 14, 14, 14, 15, 12, 12, 13,
        17, 17, 10, 18, 11, 16, 15, 10, 17, 11, 19, 13, 14, 11, 18, 10,
    ];

    const EXPECTED_SHUFFLE_BY_SORTING: [&[u8]; 45] = [
        &[0],
        &[0],
        &[0],
        &[0, 1],
        &[1, 0],
        &[1, 0],
        &[2, 0, 1],
        &[1, 0, 2],
        &[0, 2, 1],
        &[0, 1, 3, 2],
        &[0, 2, 1, 3],
        &[2, 0, 3, 1],
        &[1, 3, 2, 4, 0],
        &[1, 2, 3, 4, 0],
        &[2, 3, 0, 1, 4],
        &[5, 2, 1, 4, 3, 0],
        &[3, 5, 4, 0, 1, 2],
        &[5, 4, 3, 1, 2, 0],
        &[0, 4, 1, 5, 6, 2, 3],
        &[2, 4, 1, 6, 0, 5, 3],
        &[0, 4, 6, 1, 2, 5, 3],
        &[7, 6, 1, 3, 0, 2, 4, 5],
        &[1, 0, 5, 4, 3, 6, 2, 7],
        &[6, 0, 1, 2, 4, 7, 5, 3],
        &[4, 0, 3, 8, 7, 6, 1, 5, 2],
        &[0, 1, 7, 3, 6, 5, 4, 8, 2],
        &[5, 2, 1, 3, 8, 7, 4, 6, 0],
        &[5, 8, 1, 4, 7, 9, 3, 0, 6, 2],
        &[8, 4, 9, 0, 1, 2, 5, 3, 7, 6],
        &[8, 7, 6, 3, 1, 9, 2, 5, 4, 0],
        &[2, 1, 8, 0, 5, 9, 3, 7, 4, 10, 6],
        &[10, 5, 6, 7, 2, 3, 1, 4, 9, 8, 0],
        &[4, 1, 10, 6, 8, 9, 3, 2, 5, 0, 7],
        &[1, 4, 10, 5, 11, 7, 9, 2, 6, 8, 0, 3],
        &[8, 1, 6, 9, 4, 3, 2, 10, 5, 7, 0, 11],
        &[5, 0, 3, 1, 10, 9, 4, 6, 7, 2, 8, 11],
        &[7, 1, 6, 0, 2, 11, 3, 9, 8, 12, 10, 4, 5],
        &[6, 7, 8, 10, 4, 1, 12, 0, 5, 9, 2, 11, 3],
        &[4, 3, 6, 2, 12, 8, 5, 9, 0, 11, 7, 10, 1],
        &[5, 4, 10, 8, 9, 0, 3, 2, 7, 1, 6, 13, 12, 11],
        &[7, 11, 6, 13, 8, 10, 12, 4, 9, 3, 2, 1, 5, 0],
        &[12, 5, 7, 3, 11, 8, 9, 6, 1, 4, 0, 13, 10, 2],
        &[14, 1, 6, 5, 13, 8, 3, 0, 10, 2, 11, 9, 4, 12, 7],
        &[4, 13, 7, 0, 11, 8, 9, 14, 3, 12, 5, 10, 1, 6, 2],
        &[8, 9, 10, 7, 14, 3, 13, 0, 11, 4, 2, 1, 12, 6, 5],
    ];

    const EXPECTED_SHUFFLE_FISHER_YATES: [&[u8]; 45] = [
        &[0],
        &[0],
        &[0],
        &[0, 1],
        &[1, 0],
        &[1, 0],
        &[0, 2, 1],
        &[1, 0, 2],
        &[0, 1, 2],
        &[1, 0, 3, 2],
        &[2, 0, 3, 1],
        &[1, 3, 2, 0],
        &[0, 1, 3, 4, 2],
        &[4, 3, 1, 2, 0],
        &[1, 3, 4, 2, 0],
        &[4, 2, 3, 1, 0, 5],
        &[0, 4, 3, 2, 1, 5],
        &[2, 5, 3, 0, 4, 1],
        &[3, 4, 5, 2, 0, 1, 6],
        &[3, 1, 2, 0, 4, 5, 6],
        &[3, 0, 1, 2, 5, 4, 6],
        &[6, 2, 1, 7, 4, 5, 3, 0],
        &[0, 1, 3, 5, 6, 2, 4, 7],
        &[1, 6, 2, 7, 5, 4, 3, 0],
        &[1, 6, 0, 3, 4, 8, 5, 2, 7],
        &[6, 8, 4, 7, 2, 3, 5, 0, 1],
        &[2, 7, 0, 6, 3, 5, 4, 8, 1],
        &[4, 5, 9, 8, 3, 0, 2, 7, 6, 1],
        &[9, 7, 8, 5, 2, 4, 3, 6, 1, 0],
        &[5, 1, 7, 3, 0, 6, 8, 2, 4, 9],
        &[2, 6, 10, 5, 3, 9, 1, 4, 8, 0, 7],
        &[1, 2, 6, 7, 9, 3, 0, 8, 5, 10, 4],
        &[9, 7, 3, 5, 0, 4, 8, 1, 6, 2, 10],
        &[5, 7, 3, 9, 6, 4, 10, 8, 11, 0, 1, 2],
        &[7, 0, 2, 8, 10, 6, 1, 11, 3, 4, 5, 9],
        &[2, 7, 3, 9, 8, 11, 5, 0, 4, 6, 1, 10],
        &[12, 5, 11, 1, 3, 7, 4, 8, 2, 9, 6, 0, 10],
        &[4, 8, 10, 1, 0, 9, 2, 6, 12, 5, 7, 3, 11],
        &[10, 6, 7, 1, 5, 8, 3, 0, 4, 11, 9, 12, 2],
        &[0, 10, 11, 5, 3, 4, 1, 12, 8, 9, 13, 6, 2, 7],
        &[11, 12, 2, 7, 10, 13, 1, 0, 5, 3, 9, 8, 4, 6],
        &[9, 1, 10, 3, 5, 7, 8, 4, 11, 0, 2, 6, 13, 12],
        &[7, 13, 2, 3, 9, 6, 10, 11, 14, 0, 1, 12, 4, 8, 5],
        &[4, 5, 11, 7, 2, 10, 3, 0, 1, 13, 6, 8, 9, 12, 14],
        &[5, 8, 12, 7, 3, 10, 11, 1, 4, 0, 13, 2, 14, 6, 9],
    ];
}
