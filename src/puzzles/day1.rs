use super::Puzzle;

pub struct Day1;

impl Day1 {
    fn parse(&self, input: &str) {

    }
}

impl Puzzle for Day1 {
    type Output = i32;

    fn part1(&self, input: &str) -> Self::Output {
        0
    }

    fn part2(&self, input: &str) -> Self::Output {
        0
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 1 Part 1:  {:#?}", ans1);

        let ans2 = self.part2(&input);
        println!("Answer of Day 1 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

    #[test]
    fn test_puzzle_day1_parse() {

    }

    #[test]
    fn test_puzzle_day1_part1() {
        let puzzle = Day1;
        assert_eq!(puzzle.part1(&TESTCASE), 3);
    }

    #[test]
    fn test_puzzle_day1_part2() {
        let puzzle = Day1;
        assert_eq!(puzzle.part2(&TESTCASE), 0);
    }
}
