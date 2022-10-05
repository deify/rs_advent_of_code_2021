use itertools::Itertools;
use std::collections::HashMap;
use std::convert::From;
use std::ffi::OsString;
use std::hash::{Hash, Hasher};

const ZERO: &str = "abcefg";
const ONE: &str = "cf";
const TWO: &str = "acdeg";
const THREE: &str = "acdfg";
const FOUR: &str = "bcdf";
const FIVE: &str = "abdfg";
const SIX: &str = "abdefg";
const SEVEN: &str = "acf";
const EIGHT: &str = "abcdefg";
const NINE: &str = "abcdfg";

#[derive(Debug, Clone)]
pub struct Pattern(String);

impl std::cmp::PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }

        self.0
            .chars()
            .sorted()
            .zip(other.0.chars().sorted())
            .all(|(x, y)| x == y)
    }
}
impl Eq for Pattern {}
impl Hash for Pattern {
    fn hash<H: Hasher>(&self, hasher: &mut H) {
        let sorted: String = self.0.chars().sorted().collect();
        sorted.hash(hasher);
    }
}

impl Pattern {
    fn common_elements(&self, other: &Self) -> usize {
        let mut map: HashMap<char, usize> = HashMap::new();
        for c in self.0.chars().chain(other.0.chars()) {
            let entry = map.entry(c).or_insert(0);
            *entry += 1;
        }
        map.values().filter(|x| **x > 1).count()
    }
}

#[derive(Debug, PartialEq)]
pub struct SignalNote {
    signal_pattern: Vec<Pattern>,
    output: Vec<Pattern>,
}

impl From<&str> for SignalNote {
    fn from(string: &str) -> Self {
        let (pattern, output) = string.split_once('|').unwrap();

        SignalNote {
            signal_pattern: pattern
                .trim()
                .split(' ')
                .map(|x| Pattern(x.to_owned()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
            output: output
                .trim()
                .split(' ')
                .map(|x| Pattern(x.to_owned()))
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}
impl SignalNote {
    fn decode(&self) -> usize {
        let mut signal_pattern = self.signal_pattern.clone();

        const NONE: Option<Pattern> = None;

        let mut pattern: [Option<Pattern>; 10] = [NONE; 10];

        // first assign the ones that have a unique size
        if let Some(index) = signal_pattern.iter().position(|x| x.0.len() == ONE.len()) {
            pattern[1] = Some(signal_pattern.swap_remove(index));
        }

        if let Some(index) = signal_pattern.iter().position(|x| x.0.len() == FOUR.len()) {
            pattern[4] = Some(signal_pattern.swap_remove(index));
        }

        if let Some(index) = signal_pattern.iter().position(|x| x.0.len() == SEVEN.len()) {
            pattern[7] = Some(signal_pattern.swap_remove(index));
        }

        if let Some(index) = signal_pattern.iter().position(|x| x.0.len() == EIGHT.len()) {
            pattern[8] = Some(signal_pattern.swap_remove(index));
        }

        // assign the ones with length 5 which are TWO, THREE and FIVE
        let mut fives: Vec<_> = signal_pattern
            .iter()
            .filter(|x| x.0.len() == 5)
            .map(|x| x.to_owned())
            .collect();

        // delete fives from original signal_pattern
        signal_pattern.retain(|x| !fives.iter().any(|y| y == x));

        // THREE has 3 elements in common with SEVEN
        if let Some(index) = fives
            .iter()
            .position(|x| x.common_elements(pattern[7].as_ref().unwrap()) == 3)
        {
            pattern[3] = Some(fives.swap_remove(index));
        }

        // FIVE has 3 elements in common with FOUR
        if let Some(index) = fives
            .iter()
            .position(|x| x.common_elements(pattern[4].as_ref().unwrap()) == 3)
        {
            pattern[5] = Some(fives.swap_remove(index));
        }

        // only TWO left in fives
        assert_eq!(fives.len(), 1);
        pattern[2] = Some(fives.swap_remove(0));

        // now signal_pattern only contains elements with len 6 which are ZERO, SIX and NINE
        // NINE has 3 elements in common with SEVEN and 5 elements in common with FIVE
        if let Some(index) = signal_pattern.iter().position(|x| {
            x.common_elements(pattern[7].as_ref().unwrap()) == 3
                && x.common_elements(pattern[5].as_ref().unwrap()) == 5
        }) {
            pattern[9] = Some(signal_pattern.swap_remove(index));
        }

        // SIX has 5 elements in common with FIVE, ZERO only 4
        if let Some(index) = signal_pattern
            .iter()
            .position(|x| x.common_elements(pattern[5].as_ref().unwrap()) == 5)
        {
            pattern[6] = Some(signal_pattern.swap_remove(index));
        }

        // only ZERO is left
        assert_eq!(signal_pattern.len(), 1);
        pattern[0] = Some(signal_pattern.swap_remove(0));

        // map the output to the corresponding patterns by retrieveing their array position
        self.output
            .iter()
            .map(|x| {
                // get the position of each pattern to retrieve the number
                pattern
                    .iter()
                    .position(|y| *y.as_ref().unwrap() == *x)
                    .unwrap()
                    .to_string() // and make them strings
            })
            .join("")
            .parse() // join stringified numbers and parse as usize
            .unwrap()
    }
}

#[aoc_generator(day8)]
pub fn parse(input: &str) -> Vec<SignalNote> {
    input.lines().map(|x| SignalNote::from(x)).collect()
}

#[aoc(day8, part1)]
pub fn part1(input: &[SignalNote]) -> usize {
    input
        .iter()
        .map(|x| &x.output)
        .flatten()
        .filter(|x| {
            x.0.len() == ONE.len()
                || x.0.len() == FOUR.len()
                || x.0.len() == SEVEN.len()
                || x.0.len() == EIGHT.len()
        })
        .count()
}

#[aoc(day8, part2)]
pub fn part2(input: &[SignalNote]) -> usize {
    input.iter().map(|x| dbg!(x.decode())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
    fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
    fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
    aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
    fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
    dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
    bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
    egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
    gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce";

    const SHORT_TEST_INPUT: &str =
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![SignalNote {
                signal_pattern: vec![
                    Pattern("acedgfb".to_owned()),
                    Pattern("cdfbe".to_owned()),
                    Pattern("gcdfa".to_owned()),
                    Pattern("fbcad".to_owned()),
                    Pattern("dab".to_owned()),
                    Pattern("cefabd".to_owned()),
                    Pattern("cdfgeb".to_owned()),
                    Pattern("eafb".to_owned()),
                    Pattern("cagedb".to_owned()),
                    Pattern("ab".to_owned())
                ],
                output: vec![
                    Pattern("cdfeb".to_owned()),
                    Pattern("fcadb".to_owned()),
                    Pattern("cdfeb".to_owned()),
                    Pattern("cdbaf".to_owned())
                ]
            }],
            parse(SHORT_TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(26, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(61229, part2(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_pattern_eq() {
        assert_eq!(Pattern("ab".to_owned()), Pattern("ba".to_owned()));
        assert_ne!(Pattern("ab".to_owned()), Pattern("a".to_owned()))
    }

    #[test]
    fn test_decode_1() {
        let note = SignalNote::from(SHORT_TEST_INPUT);
        assert_eq!(note.decode(), 5353);
    }
}
