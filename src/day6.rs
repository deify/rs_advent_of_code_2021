#[derive(PartialEq, Debug, Clone)]
pub struct LanternFish(u8);

impl LanternFish {
    fn day(&mut self) -> Option<LanternFish> {
        let (new_time, fish) = match self.0.checked_sub(1) {
            Some(time) => (time, None),
            None => (6, Some(LanternFish(8))),
        };
        self.0 = new_time;
        fish
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LanternFishSchool(Vec<LanternFish>);

impl LanternFishSchool {
    fn day(&mut self) {
        let mut new_fishs: Vec<LanternFish> = Vec::new();

        for fish in self.0.iter_mut() {
            if let Some(f) = fish.day() {
                new_fishs.push(f);
            }
        }

        self.0.append(&mut new_fishs);
    }
}

pub struct EfficientLanternFishSchool {
    stages: [u64; 9],
}

impl EfficientLanternFishSchool {
    fn new() -> Self {
        Self { stages: [0; 9] }
    }

    fn from(school: &LanternFishSchool) -> EfficientLanternFishSchool {
        let mut ret = EfficientLanternFishSchool::new();
        for fish in school.0.iter() {
            ret.stages[fish.0 as usize] += 1;
        }
        ret
    }

    fn day(&mut self) {
        let old0 = self.stages[0];
        for i in 0..self.stages.len() - 1 {
            self.stages[i] = self.stages[i + 1];
        }
        self.stages[6] += old0;
        self.stages[8] = old0;
    }

    fn total(&self) -> u64 {
        self.stages.iter().sum()
    }
}

#[aoc_generator(day6)]
pub fn parse(input: &str) -> LanternFishSchool {
    LanternFishSchool(
        input
            .split(',')
            .map(|x| LanternFish(x.trim().parse().unwrap()))
            .collect(),
    )
}

#[aoc(day6, part1, bruteforce)]
pub fn part1(input: &LanternFishSchool) -> usize {
    let mut school = input.clone();

    for _ in 0..80 {
        school.day();
    }
    school.0.len()
}

#[aoc(day6, part1, eff)]
pub fn part1_eff(input: &LanternFishSchool) -> u64 {
    let mut school = EfficientLanternFishSchool::from(input);

    for _ in 0..80 {
        school.day();
    }
    school.total()
}

#[aoc(day6, part2)]
pub fn part2(input: &LanternFishSchool) -> u64 {
    let mut school = EfficientLanternFishSchool::from(input);

    for _ in 0..256 {
        school.day();
    }
    school.total()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3,4,3,1,2";

    #[test]
    fn test_parse() {
        assert_eq!(
            LanternFishSchool(vec![
                LanternFish(3),
                LanternFish(4),
                LanternFish(3),
                LanternFish(1),
                LanternFish(2)
            ]),
            parse(TEST_INPUT)
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(5934, part1(&parse(TEST_INPUT)));
        assert_eq!(5934, part1_eff(&parse(TEST_INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(26984457539, part2(&parse(TEST_INPUT)));
    }
}
