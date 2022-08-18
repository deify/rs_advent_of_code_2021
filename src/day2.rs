#[derive(Debug, PartialEq)]
pub enum Direction {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl Direction {
    fn from_command(command_string: &str) -> Direction {
        let mut split = command_string.trim().splitn(2, " ");

        let dir = split.next().unwrap();
        let num: i32 = split.next().unwrap().parse().unwrap();

        match dir {
            "forward" => Direction::Forward(num),
            "down" => Direction::Down(num),
            "up" => Direction::Up(num),
            &_ => panic!("invalid direction"),
        }
    }
}

#[derive(Debug)]
struct Position {
    depth: i32,
    horizontal: i32,
    aim: i32,
}

impl Position {
    fn new() -> Position {
        Position {
            depth: 0,
            horizontal: 0,
            aim: 0,
        }
    }

    fn go1(&mut self, direction: &Direction) {
        match direction {
            Direction::Forward(num) => self.horizontal += num,
            Direction::Down(num) => self.depth += num,
            Direction::Up(num) => self.depth -= num,
        };
    }

    fn go2(&mut self, direction: &Direction) {
        match direction {
            Direction::Forward(num) => {
                self.horizontal += num;
                self.depth += self.aim * num;
            }

            Direction::Down(num) => self.aim += num,
            Direction::Up(num) => self.aim -= num,
        };
    }
}

#[aoc_generator(day2)]
pub fn parse(input: &str) -> Vec<Direction> {
    input.lines().map(|x| Direction::from_command(x)).collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &[Direction]) -> i32 {
    let mut pos = Position::new();
    for direction in input {
        pos.go1(direction);
    }

    pos.depth * pos.horizontal
}

#[aoc(day2, part2)]
pub fn part2(input: &[Direction]) -> i32 {
    let mut pos = Position::new();
    for direction in input {
        pos.go2(direction);
    }

    pos.depth * pos.horizontal
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "forward 5
    down 5
    forward 8
    up 3
    down 8
    forward 2";

    #[test]
    fn test_parse() {
        assert_eq!(
            vec![
                Direction::Forward(5),
                Direction::Down(5),
                Direction::Forward(8),
                Direction::Up(3),
                Direction::Down(8),
                Direction::Forward(2)
            ],
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(150, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(900, part2(&parse(TEST_INPUT)));
    }
}
