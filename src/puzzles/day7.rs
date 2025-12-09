use std::collections::{HashMap, HashSet};

use super::Puzzle;

pub struct Day7;

struct Input {
    enter: usize,
    splitters: Vec<Vec<usize>>,
}

mod parser {
    use nom::IResult;

    use super::Input;

    fn parse_manifolds(input: &str) -> IResult<&str, Input> {
        let (_, lines) = nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::many1(nom::character::complete::satisfy(|c| !c.is_whitespace())),
        )(input)?;

        let enter = lines[0].iter().position(|&x| x == 'S').unwrap();
        let splitters = lines[1..]
            .iter()
            .map(|line| {
                line.iter()
                    .enumerate()
                    .filter_map(|(idx, char)| match *char {
                        '^' => Some(idx),
                        _ => None,
                    })
                    .collect()
            })
            .collect();

        Ok(("", Input { enter, splitters }))
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::sequence::preceded(
            nom::multi::many0(nom::character::complete::newline),
            parse_manifolds,
        )(input)
    }
}

impl Puzzle for Day7 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let (_, Input { enter, splitters }) = parser::parse(input).unwrap();

        let mut tachyon = std::collections::HashSet::from([enter]);
        let mut split_times = 0;

        for all_splitters in &splitters {
            tachyon = tachyon
                .iter()
                .fold(std::collections::HashSet::new(), |mut acc, cur| {
                    if all_splitters.contains(cur) {
                        split_times += 1;
                        acc.insert(cur - 1);
                        acc.insert(cur + 1);
                    } else {
                        acc.insert(*cur);
                    }

                    acc
                });
        }

        split_times
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (_, Input { enter, splitters }) = parser::parse(input).unwrap();

        let last_layers = splitters.len();
        let splitters = splitters
            .iter()
            .enumerate()
            .map(|(idx, all_splitters)| {
                all_splitters
                    .iter()
                    .map(move |&splitter| (idx + 1, splitter))
                    .collect::<Vec<(usize, usize)>>()
            })
            .flatten()
            .collect::<HashSet<(usize, usize)>>();
        let mut memo = HashMap::new();

        fn split_tachyon(
            cur_pos: usize,
            cur_layer: usize,
            last_layers: usize,
            splitters: &HashSet<(usize, usize)>,
            memo: &mut HashMap<(usize, usize), i64>,
        ) -> i64 {
            if cur_layer == last_layers {
                return 1;
            }
            let cur_tachyon = (cur_layer, cur_pos);

            if let Some(&v) = memo.get(&cur_tachyon) {
                return v;
            }

            let res = if splitters.contains(&cur_tachyon) {
                split_tachyon(cur_pos - 1, cur_layer + 1, last_layers, splitters, memo)
                    + split_tachyon(cur_pos + 1, cur_layer + 1, last_layers, splitters, memo)
            } else {
                split_tachyon(cur_pos, cur_layer + 1, last_layers, splitters, memo)
            };

            memo.insert(cur_tachyon, res);
            res
        }

        split_tachyon(enter, 0, last_layers, &splitters, &mut memo)
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 7 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 7 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"
.......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_puzzle_day7_parse() {
        let (_, input) = parser::parse(TESTCASE).unwrap();

        assert_eq!(input.enter, 7);
        assert_eq!(
            input.splitters,
            vec![
                vec![],
                vec![7],
                vec![],
                vec![6, 8],
                vec![],
                vec![5, 7, 9],
                vec![],
                vec![4, 6, 10],
                vec![],
                vec![3, 5, 9, 11],
                vec![],
                vec![2, 6, 12],
                vec![],
                vec![1, 3, 5, 7, 9, 13],
                vec![]
            ]
        )
    }

    #[test]
    fn test_puzzle_day7_part1() {
        let puzzle = Day7;

        assert_eq!(puzzle.part1(TESTCASE), 21);
    }

    #[test]
    fn test_puzzle_day7_part2() {
        let puzzle = Day7;

        assert_eq!(puzzle.part2(TESTCASE), 40);
    }
}
