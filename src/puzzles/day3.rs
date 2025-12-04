use super::Puzzle;

pub struct Day3;

type Input<'a> = Vec<&'a str>;

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input<'_>> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::character::complete::alphanumeric1,
        )(input)
    }
}

impl Day3 {}

impl Puzzle for Day3 {
    type Output = u128;

    fn part1(&self, input: &str) -> Self::Output {
        let battery_groups = parser::parse(input).unwrap().1;

        battery_groups
            .iter()
            .map(|&batteries| {
                let [max1, max2] = batteries.chars().enumerate().fold([0, 0], |acc, cur| {
                    let (cur_idx, cur_char) = cur;
                    let cur_val = cur_char.to_digit(10).unwrap() as u128;
                    let [mut max1, mut max2] = acc;

                    if cur_idx < batteries.len() - 1 && cur_val > max1 {
                        max2 = 0;
                        max1 = cur_val;
                    } else {
                        max2 = max2.max(cur_val);
                    }

                    [max1, max2]
                });

                max1 * 10 + max2
            })
            .sum()
    }

    fn part2(&self, input: &str) -> Self::Output {
        let battery_groups = parser::parse(input).unwrap().1;
        battery_groups
            .iter()
            .map(|&batteries| {
                let mut joltages = vec![0; 12];
                let num_joltages = 12;
                let num_battery = batteries.len();
                for (idx, battery) in batteries.chars().enumerate() {
                    let battery_val = battery.to_digit(10).unwrap() as u128;
                    let start = if num_joltages > (num_battery - idx) {
                        num_joltages - (num_battery - idx)
                    } else {
                        0
                    };
                    let end = idx.min(num_joltages - 1);

                    for joltage_idx in start..=end {
                        if battery_val > joltages[joltage_idx] {
                            joltages[joltage_idx] = battery_val;
                            for i in joltage_idx + 1..=end {
                                joltages[i] = 0;
                            }
                            break;
                        }
                    }
                }

                joltages.iter().fold(0, |acc, cur| acc * 10 + cur)
            })
            .sum()
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 3 Part 1:  {:#?}", ans1);

        let ans2 = self.part2(&input);
        println!("Answer of Day 3 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"987654321111111
811111111111119
234234234234278
818181911112111";

    #[test]
    fn test_puzzle_day3_parse() {
        let input = parser::parse(TESTCASE).unwrap().1;
        assert_eq!(
            input,
            vec![
                "987654321111111",
                "811111111111119",
                "234234234234278",
                "818181911112111"
            ]
        );
    }

    #[test]
    fn test_puzzle_day3_part1() {
        let puzzle = Day3;
        assert_eq!(puzzle.part1(&TESTCASE), 357);
    }

    #[test]
    fn test_puzzle_day3_part2() {
        let puzzle = Day3;
        assert_eq!(puzzle.part2(&TESTCASE), 3121910778619);
    }
}
