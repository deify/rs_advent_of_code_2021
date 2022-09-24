use core::ops::Add;

use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone, Copy, Eq, Hash)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
    fn from_str(string: &str) -> Self {
        let (x, y) = string.split_once(',').unwrap();

        Self {
            x: x.trim().parse().unwrap(),
            y: y.trim().parse().unwrap(),
        }
    }
}

impl Add<(i32, i32)> for Point {
    type Output = Point;
    fn add(self, rhs: (i32, i32)) -> Self::Output {
        Point {
            x: self.x + rhs.0,
            y: self.y + rhs.1,
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Line {
    start: Point,
    end: Point,
}

#[derive(PartialEq, Debug)]
enum LineOrientation {
    Horizontal,
    Vertical,
    Diagonal,
}

struct LinePointIter {
    line: Line,
    slope: (i32, i32),
}

impl LinePointIter {
    fn new(line: Line) -> Self {
        let mut slope = ((line.end.x - line.start.x), (line.end.y - line.start.y));

        if slope.0 == 0 && slope.1 == 0 {
            panic!("0 slope is not allowed")
        }

        if slope.0 == 0 {
            // vertical
            slope = (0, if slope.1 > 0 { 1 } else { -1 });
        } else if slope.1 == 0 {
            // horizontal
            slope = (if slope.0 > 0 { 1 } else { -1 }, 0);
        } else {
            // diagonal
            if slope.0 > 1 && slope.1 > 1 {
                slope = (1, 1);
            } else if slope.0 > 1 && slope.1 < 1 {
                slope = (1, -1);
            } else if slope.0 < 1 && slope.1 > 1 {
                slope = (-1, 1);
            } else {
                slope = (-1, -1);
            }
        }

        Self { line, slope }
    }
}
impl Iterator for LinePointIter {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.line.start != self.line.end + self.slope {
            let ret = self.line.start;
            self.line.start = self.line.start + self.slope;
            Some(ret)
        } else {
            None
        }
    }
}

impl Line {
    fn new(start: Point, end: Point) -> Self {
        Self { start, end }
    }

    fn from_str(string: &str) -> Self {
        let (start, end) = string.split_once("->").unwrap();

        Self {
            start: Point::from_str(start.trim()),
            end: Point::from_str(end.trim()),
        }
    }

    fn orientation(&self) -> LineOrientation {
        if self.start.x == self.end.x {
            LineOrientation::Vertical
        } else if self.start.y == self.end.y {
            LineOrientation::Horizontal
        } else {
            LineOrientation::Diagonal
        }
    }

    fn point_iter(&self) -> LinePointIter {
        LinePointIter::new(self.clone())
    }
}

struct LineMap(HashMap<Point, usize>);

impl LineMap {
    fn draw_line(&mut self, line: &Line) {
        for point in line.point_iter() {
            let count = self.0.entry(point).or_insert(0);
            *count += 1;
        }
    }

    fn new() -> Self {
        LineMap(HashMap::new())
    }
}

#[aoc_generator(day5)]
pub fn parse(input: &str) -> Vec<Line> {
    input.lines().map(|x| Line::from_str(x.trim())).collect()
}

#[aoc(day5, part1)]
pub fn part1(input: &[Line]) -> usize {
    let mut map = LineMap::new();

    let lines = input
        .iter()
        .filter(|x| x.orientation() != LineOrientation::Diagonal);

    for line in lines {
        map.draw_line(line);
    }
    map.0.iter().filter(|(k, v)| **v > 1).count()
}

#[aoc(day5, part2)]
pub fn part2(input: &[Line]) -> usize {
    let mut map = LineMap::new();

    for line in input {
        map.draw_line(line);
    }
    map.0.iter().filter(|(k, v)| **v > 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "0,9 -> 5,9
    8,0 -> 0,8
    9,4 -> 3,4
    2,2 -> 2,1
    7,0 -> 7,4
    6,4 -> 2,0
    0,9 -> 2,9
    3,4 -> 1,4
    0,0 -> 8,8
    5,5 -> 8,2";

    #[test]
    fn test_parse() {
        let expected = vec![
            Line::new(Point::new(0, 9), Point::new(5, 9)),
            Line::new(Point::new(8, 0), Point::new(0, 8)),
            Line::new(Point::new(9, 4), Point::new(3, 4)),
            Line::new(Point::new(2, 2), Point::new(2, 1)),
            Line::new(Point::new(7, 0), Point::new(7, 4)),
            Line::new(Point::new(6, 4), Point::new(2, 0)),
            Line::new(Point::new(0, 9), Point::new(2, 9)),
            Line::new(Point::new(3, 4), Point::new(1, 4)),
            Line::new(Point::new(0, 0), Point::new(8, 8)),
            Line::new(Point::new(5, 5), Point::new(8, 2)),
        ];
        assert_eq!(expected, parse(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(5, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(12, part2(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_line_point_iter_vertical() {
        let line = Line::new(Point { x: 0, y: 0 }, Point { x: 0, y: 2 });

        assert_eq!(line.orientation(), LineOrientation::Vertical);

        let points: Vec<_> = line.point_iter().collect();
        assert_eq!(
            points,
            vec![Point::new(0, 0), Point::new(0, 1), Point::new(0, 2)]
        )
    }

    #[test]
    fn test_line_point_iter_horizontal() {
        let line = Line::new(Point { x: 0, y: 0 }, Point { x: 2, y: 0 });

        assert_eq!(line.orientation(), LineOrientation::Horizontal);

        let points: Vec<_> = line.point_iter().collect();
        assert_eq!(
            points,
            vec![Point::new(0, 0), Point::new(1, 0), Point::new(2, 0)]
        )
    }
}
