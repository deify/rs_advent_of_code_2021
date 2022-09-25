#[derive(Debug, PartialEq)]
pub struct Crab {
    position: i32,
}

impl Crab {
    fn fuel_2(&self, pos: i32) -> usize {
        (0..=((self.position - pos).abs() as usize)).sum()
    }
}

#[aoc_generator(day7)]
pub fn parse(input: &str) -> Vec<Crab> {
    input
        .split(',')
        .map(|x| Crab {
            position: x.parse().unwrap(),
        })
        .collect()
}

#[aoc(day7, part1)]
pub fn part1(input: &[Crab]) -> usize {
    let min = input.iter().map(|x| x.position).min().unwrap();
    let max = input.iter().map(|x| x.position).max().unwrap();

    (min..=max)
        .map(|pos| {
            input
                .iter()
                .map(|x| (x.position - pos).abs() as usize)
                .sum()
        })
        .min()
        .unwrap()
}

#[aoc(day7, part2)]
pub fn part2(input: &[Crab]) -> usize {
    let min = input.iter().map(|x| x.position).min().unwrap();
    let max = input.iter().map(|x| x.position).max().unwrap();

    (min..=max)
        .map(|pos| input.iter().map(|x| x.fuel_2(pos)).sum())
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                Crab { position: 16 },
                Crab { position: 1 },
                Crab { position: 2 },
                Crab { position: 0 },
                Crab { position: 4 },
                Crab { position: 2 },
                Crab { position: 7 },
                Crab { position: 1 },
                Crab { position: 2 },
                Crab { position: 14 },
            ],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(37, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(168, part2(&parse(TEST_INPUT)));
    }
}
