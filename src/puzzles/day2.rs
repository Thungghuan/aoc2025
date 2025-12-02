use super::Puzzle;

pub struct Day2;

type Input = Vec<(i64, i64)>;

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::multi::separated_list1(
            nom::bytes::complete::tag(","),
            nom::sequence::separated_pair(
                nom::character::complete::i64,
                nom::bytes::complete::tag("-"),
                nom::character::complete::i64,
            ),
        )(input)
    }
}

impl Day2 {
    fn count_digits(&self, mut number: i64) -> i64 {
        let mut cnt = 0;
        while number > 0 {
            number /= 10;
            cnt += 1;
        }

        cnt
    }

    fn check_by_repeat_2(&self, number: i64) -> bool {
        let digits = self.count_digits(number);
        if digits % 2 != 0 {
            return true;
        }

        let base = 10i64.pow(digits as u32 / 2);
        number / base != number % base
    }

    fn check_by_repeat_n(&self, number: i64) -> bool {
        let digits = self.count_digits(number);

        for seq_len in 1..=(digits / 2) {
            if digits % seq_len != 0 {
                continue;
            }

            let mut number_str = number.to_string();

            if (0..digits / seq_len)
                .fold(vec![], |mut acc, _| {
                    let next = number_str.split_off(seq_len as usize);
                    acc.push(number_str.clone());
                    number_str = next;
                    acc
                })
                .windows(2)
                .all(|w| w[0] == w[1])
            {
                return false;
            }
        }

        true
    }
}

impl Puzzle for Day2 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let (_, range_list) = parser::parse(input).unwrap();
        range_list
            .iter()
            .map(|(start, end)| {
                (*start..=*end)
                    .filter(|&n| !self.check_by_repeat_2(n))
                    .sum::<i64>()
            })
            .sum()
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (_, range_list) = parser::parse(input).unwrap();
        range_list
            .iter()
            .map(|(start, end)| {
                (*start..=*end)
                    .filter(|&n| !self.check_by_repeat_n(n))
                    .sum::<i64>()
            })
            .sum()
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

    const TESTCASE: &'static str = r"11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_puzzle_day1_parse() {
        let input = parser::parse(TESTCASE).unwrap().1;
        assert_eq!(
            input,
            vec![
                (11, 22),
                (95, 115),
                (998, 1012),
                (1188511880, 1188511890),
                (222220, 222224),
                (1698522, 1698528),
                (446443, 446449),
                (38593856, 38593862),
                (565653, 565659),
                (824824821, 824824827),
                (2121212118, 2121212124),
            ]
        );
    }

    #[test]
    fn test_is_valid() {
        let puzzle = Day2;
        assert!(!puzzle.check_by_repeat_2(1010));
        assert!(!puzzle.check_by_repeat_2(1188511885));
    }

    #[test]
    fn test_puzzle_day1_part1() {
        let puzzle = Day2;
        assert_eq!(puzzle.part1(&TESTCASE), 1227775554);
    }

    #[test]
    fn test_puzzle_day1_part2() {
        let puzzle = Day2;
        assert_eq!(puzzle.part2(&TESTCASE), 4174379265);
    }
}
