use super::Puzzle;

pub struct Day5;

type IngredientRange = Vec<(i64, i64)>;
type IngredientIDs = Vec<i64>;
struct Input {
    ingredient_range: IngredientRange,
    ingredient_ids: IngredientIDs,
}

mod parser {
    use nom::IResult;

    use crate::puzzles::day5::{IngredientIDs, IngredientRange};

    use super::Input;

    fn parse_ingredient_range(input: &str) -> IResult<&str, IngredientRange> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::sequence::separated_pair(
                nom::character::complete::i64,
                nom::bytes::complete::tag("-"),
                nom::character::complete::i64,
            ),
        )(input)
    }

    fn parse_ingredient_ids(input: &str) -> IResult<&str, IngredientIDs> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::character::complete::i64,
        )(input)
    }

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::combinator::map_res(
            nom::sequence::separated_pair(
                parse_ingredient_range,
                nom::bytes::complete::tag("\n\n"),
                parse_ingredient_ids,
            ),
            |(ingredient_range, ingredient_ids)| {
                Ok::<Input, &str>(Input {
                    ingredient_range,
                    ingredient_ids,
                })
            },
        )(input)
    }
}

impl Day5 {
    fn merge_ingredient_range(&self, ingredient_range: &mut IngredientRange) -> IngredientRange {
        ingredient_range.sort_by(|a, b| {
            if a.0 == b.0 {
                a.1.cmp(&b.1)
            } else {
                a.0.cmp(&b.0)
            }
        });

        let mut merged_range: IngredientRange = vec![];
        for (start, end) in ingredient_range {
            if let Some(last_range) = merged_range.pop() {
                if *start <= last_range.1 {
                    merged_range.push((last_range.0, last_range.1.max(*end)));
                } else {
                    merged_range.push(last_range);
                    merged_range.push((*start, *end))
                }
            } else {
                merged_range.push((*start, *end));
            }
        }

        merged_range
    }
}

impl Puzzle for Day5 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let (
            _,
            Input {
                mut ingredient_range,
                ingredient_ids,
            },
        ) = parser::parse(input).unwrap();

        let merge_ingredient_range = self.merge_ingredient_range(&mut ingredient_range);

        ingredient_ids
            .iter()
            .filter(|&id| {
                merge_ingredient_range
                    .iter()
                    .filter(|(start, end)| id >= start && id <= end)
                    .count()
                    > 0
            })
            .count() as i64
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (
            _,
            Input {
                mut ingredient_range,
                ingredient_ids: _,
            },
        ) = parser::parse(input).unwrap();

        let merge_ingredient_range = self.merge_ingredient_range(&mut ingredient_range);
        merge_ingredient_range
            .iter()
            .map(|(start, end)| *end - *start + 1)
            .sum()
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 5 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 5 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_puzzle_day5_parse() {
        let input = parser::parse(TESTCASE).unwrap().1;
        assert_eq!(
            input.ingredient_range,
            vec![(3, 5), (10, 14), (16, 20), (12, 18)]
        );
        assert_eq!(input.ingredient_ids, vec![1, 5, 8, 11, 17, 32])
    }

    #[test]
    fn test_puzzle_day5_part1() {
        let puzzle = Day5;

        assert_eq!(puzzle.part1(TESTCASE), 3);
    }

    #[test]
    fn test_puzzle_day5_part2() {
        let puzzle = Day5;

        assert_eq!(puzzle.part2(TESTCASE), 14);
    }
}
