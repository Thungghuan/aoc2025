use super::Puzzle;

pub struct Day1;

#[derive(PartialEq, Eq, Debug, Clone, Copy)]
enum Rotation {
    LEFT,
    RIGHT,
}

impl Day1 {
    fn parse(&self, input: &str) -> Vec<(Rotation, i32)> {
        input
            .trim()
            .split("\n")
            .map(|instruction| {
                let rotation = match &instruction[0..1] {
                    "L" => Rotation::LEFT,
                    "R" => Rotation::RIGHT,
                    _ => unreachable!(),
                };
                let distance = instruction[1..].parse().unwrap();

                (rotation, distance)
            })
            .collect()
    }
}

impl Puzzle for Day1 {
    type Output = i32;

    fn part1(&self, input: &str) -> Self::Output {
        self.parse(input)
            .iter()
            .fold((50, 0), |acc, x| {
                let next = match x.0 {
                    Rotation::LEFT => (acc.0 - x.1) % 100,
                    Rotation::RIGHT => (acc.0 + x.1) % 100,
                };

                (next, acc.1 + if next == 0 { 1 } else { 0 })
            })
            .1
    }

    fn part2(&self, input: &str) -> Self::Output {
        self.parse(input)
            .iter()
            .fold((50, 0), |acc, x| {
                let difference = x.1 % 100;
                let cycle = x.1 / 100;

                let next = match x.0 {
                    Rotation::LEFT => (
                        (acc.0 - difference + 100) % 100,
                        (acc.0 <= difference && acc.0 > 0),
                    ),
                    Rotation::RIGHT => ((acc.0 + difference) % 100, acc.0 + difference >= 100),
                };

                let zeros = cycle + if next.1 { 1 } else { 0 };

                (next.0, acc.1 + zeros)
            })
            .1
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
        let puzzle = Day1;
        let instructions = puzzle.parse(TESTCASE);
        assert_eq!(
            instructions,
            vec![
                (Rotation::LEFT, 68),
                (Rotation::LEFT, 30),
                (Rotation::RIGHT, 48),
                (Rotation::LEFT, 5),
                (Rotation::RIGHT, 60),
                (Rotation::LEFT, 55),
                (Rotation::LEFT, 1),
                (Rotation::LEFT, 99),
                (Rotation::RIGHT, 14),
                (Rotation::LEFT, 82)
            ]
        )
    }

    #[test]
    fn test_puzzle_day1_part1() {
        let puzzle = Day1;
        assert_eq!(puzzle.part1(&TESTCASE), 3);
    }

    #[test]
    fn test_puzzle_day1_part2() {
        let puzzle = Day1;
        assert_eq!(puzzle.part2(&TESTCASE), 6);
    }
}
