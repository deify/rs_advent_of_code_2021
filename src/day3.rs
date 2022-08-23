use grid;

enum BinaryMost {
    False,
    True,
    Equal,
}

impl BinaryMost {
    fn from_iter<'a>(iter: impl Iterator<Item = &'a bool>) -> BinaryMost {
        let mut true_count = 0;
        let mut false_count = 0;
        for b in iter {
            match b {
                false => false_count += 1,
                true => true_count += 1,
            }
        }

        if true_count > false_count {
            BinaryMost::True
        } else if false_count > true_count {
            BinaryMost::False
        } else {
            BinaryMost::Equal
        }
    }
}
impl TryInto<bool> for BinaryMost {
    type Error = &'static str;
    fn try_into(self) -> Result<bool, Self::Error> {
        match self {
            BinaryMost::False => Ok(false),
            BinaryMost::True => Ok(true),
            BinaryMost::Equal => Err("Equal can not be converted to bool"),
        }
    }
}

fn bool_iter_to_string(bool_iter: impl Iterator<Item = bool>) -> String {
    bool_iter.map(|x| if x { '1' } else { '0' }).collect()
}

#[aoc_generator(day3)]
pub fn parse(input: &str) -> grid::Grid<bool> {
    let cols = input.lines().next().unwrap().trim().len();
    let vec: Vec<bool> = input
        .lines()
        .map(|l| l.trim().chars().map(|c| c == '1'))
        .flatten()
        .collect();

    grid::Grid::from_vec(vec, cols)
}

#[aoc(day3, part1)]
pub fn part1(input: &grid::Grid<bool>) -> usize {
    let gamma_str: String = bool_iter_to_string(
        (0..input.cols()).map(|x| BinaryMost::from_iter(input.iter_col(x)).try_into().unwrap()),
    );

    let epsilon_str: String = bool_iter_to_string((0..input.cols()).map(|x| {
        !(<BinaryMost as TryInto<bool>>::try_into(BinaryMost::from_iter(input.iter_col(x)))
            .unwrap())
    }));

    let gamma = usize::from_str_radix(&gamma_str, 2).unwrap();
    let epsilon = usize::from_str_radix(&epsilon_str, 2).unwrap();

    gamma * epsilon
}

fn filter_rate<T>(grid: &grid::Grid<bool>, filter: T) -> usize
where
    T: Fn(&bool, &BinaryMost) -> bool,
{
    let mut use_rows: Vec<_> = (0..grid.rows()).collect();

    for col in 0..grid.cols() {
        // get the most common bit in current column
        let temp_grid = grid::Grid::from_vec(
            use_rows
                .iter()
                .map(|x| grid.iter_row(*x).map(|x| *x))
                .flatten()
                .collect(),
            grid.cols(),
        );

        let most_common = BinaryMost::from_iter(temp_grid.iter_col(col));

        let mut new_use_rows = use_rows.clone();
        for row in use_rows.iter() {
            let row_vec: Vec<_> = grid.iter_row(*row).collect();
            let element = row_vec[col];
            if !filter(element, &most_common) {
                new_use_rows.remove(new_use_rows.binary_search(row).unwrap());
                if new_use_rows.len() == 1 {
                    break;
                }
            }
        }
        use_rows = new_use_rows;
        if use_rows.len() == 1 {
            break;
        }
    }
    use_rows[0]
}

#[aoc(day3, part2)]
pub fn part2(input: &grid::Grid<bool>) -> usize {
    let oxygen_rate = filter_rate(input, |value, most_common| match most_common {
        BinaryMost::False => !*value,
        BinaryMost::True => *value,
        BinaryMost::Equal => *value,
    });

    let oxygen_rate_str: String = bool_iter_to_string(input.iter_row(oxygen_rate).map(|x| *x));

    println!("oxygen rating {}", &oxygen_rate_str);

    let co2_rate = filter_rate(input, |value, most_common| match most_common {
        BinaryMost::False => *value,
        BinaryMost::True => !*value,
        BinaryMost::Equal => !*value,
    });

    let co2_rate_str: String = bool_iter_to_string(input.iter_row(co2_rate).map(|x| *x));
    println!("co2 scrubber rate {}", &co2_rate_str);

    let oxygen = usize::from_str_radix(&oxygen_rate_str, 2).unwrap();
    let co2 = usize::from_str_radix(&co2_rate_str, 2).unwrap();

    oxygen * co2
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_INPUT: &str = "00100
    11110
    10110
    10111
    10101
    01111
    00111
    11100
    10000
    11001
    00010
    01010";

    const SHORT_TEST_INPUT: &str = "00100
        11110
        10110
        10111";

    #[test]
    fn test_parse() {
        let expected = grid::grid![
            [false, false, true, false, false]
            [true, true, true, true, false]
            [true, false, true, true, false]
            [true, false, true, true, true]
        ];
        assert_eq!(expected, parse(&SHORT_TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(198, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(230, part2(&parse(TEST_INPUT)));
    }
}
