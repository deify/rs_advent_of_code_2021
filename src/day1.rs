#[aoc_generator(day1)]
pub fn parse(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &[i32]) -> usize {
    let num_iter = input.iter();

    num_iter
        .clone()
        .zip(num_iter.skip(1))
        .map(|(a, b)| b - a)
        .filter(|x| *x > 0)
        .count()
}

#[aoc(day1, part1, windows)]
pub fn part1_windows(input: &[i32]) -> usize {
    input
        .windows(2)
        .map(|x| x[1] - x[0])
        .filter(|x| *x > 0)
        .count()
}

#[aoc(day1, part2)]
pub fn part2(input: &[i32]) -> usize {
    let window_sums: Vec<i32> = input.windows(3).map(|x| x.iter().sum()).collect();

    window_sums
        .windows(2)
        .map(|x| x[1] - x[0])
        .filter(|x: &i32| *x > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "199
        200
        208
        210
        200
        207
        240
        269
        260
        263";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        let input = parse(TEST_INPUT);
        assert_eq!(7, part1(&input));
        assert_eq!(7, part1_windows(&input));
    }
    #[test]
    fn test_part2() {
        assert_eq!(5, part2(&parse(TEST_INPUT)));
    }
}
