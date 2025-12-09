use super::Puzzle;

pub struct Day6;

struct Input {
    nums: Vec<Vec<i64>>,
    ops: Vec<char>,
}

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        let (_, mut lines) = nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::character::complete::not_line_ending,
        )(input.trim())?;

        let ops_row = lines.pop().unwrap();
        let nums = lines
            .iter()
            .map(|&row| {
                nom::multi::separated_list1(
                    nom::character::complete::multispace1::<_, nom::error::Error<&str>>,
                    nom::character::complete::i64,
                )(row.trim())
                .unwrap()
                .1
            })
            .fold(vec![], |mut acc, cur| {
                if acc.len() == 0 {
                    for _ in 0..cur.len() {
                        acc.push(vec![]);
                    }
                }
                cur.iter()
                    .enumerate()
                    .for_each(|(idx, val)| acc[idx].push(*val));

                acc
            });
        let ops = nom::multi::separated_list1(
            nom::character::complete::multispace1::<_, nom::error::Error<&str>>,
            nom::character::complete::satisfy(|c| !c.is_whitespace()),
        )(ops_row.trim())
        .unwrap()
        .1;

        Ok(("", Input { nums, ops }))
    }

    pub fn parse_cephalopod(input: &str) -> IResult<&str, Input> {
        let (_, mut lines) = nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::character::complete::not_line_ending,
        )(input.trim())?;

        let ops_row = lines.pop().unwrap();
        let ops = nom::multi::separated_list1(
            nom::character::complete::multispace1::<_, nom::error::Error<&str>>,
            nom::character::complete::satisfy(|c| !c.is_whitespace()),
        )(ops_row.trim())
        .unwrap()
        .1
        .iter()
        .copied()
        .rev()
        .collect();

        let nums_chars_rev = lines
            .iter()
            .map(|row| row.chars().rev().collect::<Vec<char>>())
            .fold(vec![], |mut acc, cur| {
                if acc.len() == 0 {
                    for _ in 0..cur.len() {
                        acc.push(vec![]);
                    }
                }
                cur.iter()
                    .enumerate()
                    .for_each(|(idx, val)| acc[idx].push(*val));

                acc
            });

        let cephalopod_nums: Vec<Vec<i64>> = nums_chars_rev
            .split(|chars| chars.iter().all(|c| *c == ' '))
            .map(|nums| {
                nums.iter()
                    .map(|num_chars| {
                        num_chars.iter().fold(0, |acc, cur| {
                            if *cur == ' ' {
                                acc
                            } else {
                                acc * 10 + cur.to_digit(10).unwrap() as i64
                            }
                        })
                    })
                    .collect::<Vec<i64>>()
            })
            .collect();

        Ok((
            "",
            Input {
                nums: cephalopod_nums,
                ops,
            },
        ))
    }
}

impl Puzzle for Day6 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let (_, Input { nums, ops }) = parser::parse(input).unwrap();

        nums.iter()
            .zip(ops)
            .map(|(arr, op)| match op {
                '+' => arr.iter().sum::<i64>(),
                '*' => arr.iter().product::<i64>(),
                _ => unreachable!(),
            })
            .sum()
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (_, Input { nums, ops }) = parser::parse_cephalopod(input).unwrap();

        nums.iter()
            .zip(ops)
            .map(|(arr, op)| match op {
                '+' => arr.iter().sum::<i64>(),
                '*' => arr.iter().product::<i64>(),
                _ => unreachable!(),
            })
            .sum()
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 6 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 6 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_puzzle_day6_parse() {
        let input = parser::parse(TESTCASE).unwrap().1;

        assert_eq!(
            input.nums,
            vec![[123, 45, 6], [328, 64, 98], [51, 387, 215], [64, 23, 314]]
        );

        assert_eq!(input.ops, vec!['*', '+', '*', '+']);
    }

    #[test]
    fn test_puzzle_day6_part1() {
        let puzzle = Day6;

        assert_eq!(puzzle.part1(TESTCASE), 4277556);
    }

    #[test]
    fn test_puzzle_day6_part2() {
        let puzzle = Day6;

        assert_eq!(puzzle.part2(TESTCASE), 3263827);
    }
}
