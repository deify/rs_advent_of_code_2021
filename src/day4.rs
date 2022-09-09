use grid::Grid;
use std::{collections::VecDeque, vec};

#[derive(Debug, PartialEq, Eq, Clone)]
struct BingoField {
    number: i32,
    checked: bool,
}

impl BingoField {
    fn check(&mut self) {
        self.checked = true;
    }

    fn checked(&self) -> bool {
        return self.checked;
    }

    fn new(number: i32) -> BingoField {
        BingoField {
            number: number,
            checked: false,
        }
    }
}

#[derive(PartialEq)]
pub enum BingoResult {
    Row(usize),
    Column(usize),
    NoBingo,
}

const BINGO_BOARD_SIZE: usize = 5;

#[derive(Debug, PartialEq, Clone)]
pub struct BingoBoard {
    board: Grid<BingoField>,
}

impl BingoBoard {
    fn check(&mut self, number: i32) {
        match self.board.iter_mut().find(|x| x.number == number) {
            Some(x) => x.check(),
            None => {}
        }
    }

    fn bingo(&self) -> BingoResult {
        for row in 0..self.board.rows() {
            if self.board.iter_row(row).all(|x| x.checked()) {
                return BingoResult::Row(row);
            }
        }

        for col in 0..self.board.cols() {
            if self.board.iter_col(col).all(|x| x.checked()) {
                return BingoResult::Column(col);
            }
        }

        BingoResult::NoBingo
    }

    fn from_str(str: &str, board_size: usize) -> BingoBoard {
        let vec: Vec<_> = str
            .lines()
            .map(|x| {
                x.trim()
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .map(|x| BingoField::new(x.parse().unwrap()))
            })
            .flatten()
            .collect();
        let grid = Grid::from_vec(vec, board_size);
        assert_eq!(board_size, grid.rows());

        BingoBoard { board: grid }
    }

    fn new(grid: Grid<i32>) -> BingoBoard {
        let board = Grid::from_vec(
            grid.iter().map(|x| BingoField::new(*x)).collect(),
            grid.cols(),
        );
        BingoBoard { board: board }
    }

    fn sum_unchecked(&self) -> i32 {
        self.board
            .iter()
            .filter(|x| !x.checked())
            .map(|x| x.number)
            .sum()
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct BingoInput {
    numbers: VecDeque<i32>,
    bingo_boards: Vec<BingoBoard>,
}

impl BingoInput {
    fn get_winner_iter(&self) -> BingoWinnerIter {
        BingoWinnerIter {
            bingo_input: self.clone(),
        }
    }
}

struct BingoWinnerIter {
    bingo_input: BingoInput,
}

impl Iterator for BingoWinnerIter {
    type Item = (i32, Vec<BingoBoard>);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.bingo_input.bingo_boards.is_empty() {
                return None;
            }

            if let Some(num) = self.bingo_input.numbers.pop_front() {
                for board in self.bingo_input.bingo_boards.iter_mut() {
                    board.check(num);
                }

                let mut winner_boards: Vec<BingoBoard> = Vec::new();
                let mut i = 0;

                while i < self.bingo_input.bingo_boards.len() {
                    if let BingoResult::Column(_) | BingoResult::Row(_) =
                        self.bingo_input.bingo_boards[i].bingo()
                    {
                        winner_boards.push(self.bingo_input.bingo_boards.remove(i));
                    } else {
                        i += 1;
                    }
                }

                if !winner_boards.is_empty() {
                    return Some((num, winner_boards));
                }
            } else {
                return None;
            }
        }
    }
}

#[aoc_generator(day4)]
pub fn parse(input: &str) -> BingoInput {
    let mut line_iter = input.lines();

    let numbers: VecDeque<i32> = line_iter
        .next()
        .unwrap()
        .trim()
        .split(",")
        .map(|x| {
            x.trim()
                .parse()
                .unwrap_or_else(|err| panic!("Failed to parse {x} with err {err}"))
        })
        .collect();

    let mut boards: Vec<BingoBoard> = vec![];

    while line_iter.next().is_some() {
        let board_str: String = line_iter
            .by_ref()
            .take(BINGO_BOARD_SIZE)
            .collect::<Vec<_>>()
            .join("\n");

        boards.push(BingoBoard::from_str(&board_str, BINGO_BOARD_SIZE));
    }

    BingoInput {
        numbers: numbers,
        bingo_boards: boards,
    }
}

#[aoc(day4, part1)]
pub fn part1(input: &BingoInput) -> i32 {
    let mut winner_iter = input.get_winner_iter();

    if let Some((winning_number, boards)) = winner_iter.next() {
        assert_eq!(1, boards.len());
        boards[0].sum_unchecked() * winning_number
    } else {
        -1
    }
}

#[aoc(day4, part2)]
pub fn part2(input: &BingoInput) -> i32 {
    let winner_iter = input.get_winner_iter();

    if let Some((winning_number, boards)) = winner_iter.last() {
        assert_eq!(1, boards.len());
        boards[0].sum_unchecked() * winning_number
    } else {
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str =
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

    22 13 17 11  0
     8  2 23  4 24
    21  9 14 16  7
     6 10  3 18  5
     1 12 20 15 19
    
     3 15  0  2 22
     9 18 13 17  5
    19  8  7 25 23
    20 11 10 24  4
    14 21 16 12  6
    
    14 21 17 24  4
    10 16 15  9 19
    18  8 23 26 20
    22 11 13  6  5
     2  0 12  3  7";

    #[test]
    fn test_parse() {
        let expected = BingoInput {
            numbers: VecDeque::from(vec![
                7, 4, 9, 5, 11, 17, 23, 2, 0, 14, 21, 24, 10, 16, 13, 6, 15, 25, 12, 22, 18, 20, 8,
                19, 3, 26, 1,
            ]),
            bingo_boards: vec![
                BingoBoard::new(grid::grid![
                [22, 13, 17, 11,  0]
                [ 8,  2, 23,  4, 24]
                [21,  9, 14, 16,  7]
                [ 6, 10,  3, 18,  5]
                [ 1, 12, 20, 15, 19]
                ]),
                BingoBoard::new(grid::grid![
                [ 3, 15,  0,  2, 22]
                [ 9, 18, 13, 17,  5]
                [19,  8,  7, 25, 23]
                [20, 11, 10, 24,  4]
                [14, 21, 16, 12,  6]
                ]),
                BingoBoard::new(grid::grid![
                [14, 21, 17, 24,  4]
                [10, 16, 15,  9, 19]
                [18,  8, 23, 26, 20]
                [22, 11, 13,  6,  5]
                [ 2,  0, 12,  3,  7]
                ]),
            ],
        };
        assert_eq!(expected, parse(TEST_INPUT));
    }

    #[test]
    fn test_part1() {
        assert_eq!(4512, part1(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(1924, part2(&parse(TEST_INPUT)));
    }
}
