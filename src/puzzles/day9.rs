use super::Puzzle;

pub struct Day9;

type Position = (i64, i64);
type Input = Vec<Position>;

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::sequence::separated_pair(
                nom::character::complete::i64,
                nom::bytes::complete::tag(","),
                nom::character::complete::i64,
            ),
        )(input)
    }
}

impl Day9 {
    fn count_area(&self, point_a: Position, point_b: Position) -> i64 {
        ((point_a.0 - point_b.0).abs() + 1) * ((point_a.1 - point_b.1).abs() + 1)
    }

    fn check_point_inside(&self, point: &Position, tiles: &Vec<Position>) -> bool {
        let (x, y) = *point;
        let mut is_inside = false;

        for i in 0..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[(i + 1) % tiles.len()];

            if (x1 == x2 && x1 == x) && ((y1 > y) != (y2 > y)) {
                return true;
            }
            if (y1 == y2 && y1 == y) && ((x1 > x) != (x2 > x)) {
                return true;
            }
        }

        for i in 0..tiles.len() {
            let (x1, y1) = tiles[i];
            let (_x2, y2) = tiles[(i + 1) % tiles.len()];

            if (y1 > y) != (y2 > y) {
                if x1 > x {
                    is_inside = !is_inside;
                }
            }
        }

        is_inside
    }
}

impl Puzzle for Day9 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let (_, tiles) = parser::parse(input).unwrap();
        let mut max_ares = 0;

        for i in 0..tiles.len() {
            for j in 0..i {
                max_ares = max_ares.max(self.count_area(tiles[i], tiles[j]));
            }
        }

        max_ares
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (_, tiles) = parser::parse(input).unwrap();
        let mut max_ares = 0;

        for i in 0..tiles.len() {
            for j in 0..i {
                let (x1, y1) = tiles[i];
                let (x2, y2) = tiles[j];

                let dx = (x1 - x2).abs();
                let x1 = x1.min(x2);
                let x2 = x1 + dx;

                let dy = (y1 - y2).abs();
                let y1 = y1.min(y2);
                let y2 = y1 + dy;

                let mut is_valid = true;
                for x in x1..=x2 {
                    for y in y1..=y2 {
                        if !self.check_point_inside(&(x, y), &tiles) {
                            is_valid = false;
                            break;
                        }
                    }
                    if !is_valid {
                        break;
                    }
                }

                if is_valid {
                    max_ares = max_ares.max(self.count_area(tiles[i], tiles[j]));
                }
            }
        }

        max_ares
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 9 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 9 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    #[test]
    fn test_puzzle_day9_parse() {
        let (_, input) = parser::parse(TESTCASE).unwrap();

        assert_eq!(
            input,
            vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]
        );
    }

    #[test]
    fn test_puzzle_day9_part1() {
        let puzzle = Day9;

        assert_eq!(puzzle.part1(TESTCASE), 50);
    }

    #[test]
    fn test_puzzle_day9_part2() {
        let puzzle = Day9;

        assert_eq!(puzzle.part2(TESTCASE), 24);
    }
}
