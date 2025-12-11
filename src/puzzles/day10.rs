use super::Puzzle;

pub struct Day10;

struct Machine {
    indicator_lights: Vec<bool>,
    button_schematics: Vec<Vec<usize>>,
    joltages: Vec<i64>,
}

type Input = Vec<Machine>;

mod parser {
    use nom::IResult;

    use crate::puzzles::day10::Machine;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::combinator::map(
            nom::multi::separated_list1(
                nom::character::complete::newline,
                nom::multi::separated_list1(
                    nom::character::complete::space1,
                    nom::combinator::map(nom::bytes::complete::is_not(" \n"), |s: &str| {
                        s.to_string()
                    }),
                ),
            ),
            |res| {
                res.iter()
                    .map(|machine| Machine {
                        indicator_lights: parse_indicator_lights(&machine[0]).unwrap().1,
                        button_schematics: machine[1..machine.len() - 1]
                            .iter()
                            .map(|m| parser_button_schematics(m).unwrap().1)
                            .collect(),
                        joltages: parse_joltages(&machine[machine.len() - 1]).unwrap().1,
                    })
                    .collect()
            },
        )(input)
    }

    fn parse_indicator_lights(input: &str) -> IResult<&str, Vec<bool>> {
        nom::sequence::delimited(
            nom::character::complete::char('['),
            nom::multi::many1(nom::combinator::map(
                nom::character::complete::one_of("#."),
                |c| c == '#',
            )),
            nom::character::complete::char(']'),
        )(input)
    }

    fn parser_button_schematics(input: &str) -> IResult<&str, Vec<usize>> {
        nom::sequence::delimited(
            nom::character::complete::char('('),
            nom::multi::separated_list0(
                nom::bytes::complete::tag(","),
                nom::combinator::map(nom::character::complete::i64, |n| n as usize),
            ),
            nom::character::complete::char(')'),
        )(input)
    }

    fn parse_joltages(input: &str) -> IResult<&str, Vec<i64>> {
        nom::sequence::delimited(
            nom::character::complete::char('{'),
            nom::multi::separated_list0(
                nom::bytes::complete::tag(","),
                nom::character::complete::i64,
            ),
            nom::character::complete::char('}'),
        )(input)
    }
}

impl Day10 {
    fn convert_light_to_number(&self, indicator_lights: &Vec<bool>) -> i32 {
        indicator_lights
            .iter()
            .rev()
            .fold(0, |acc, cur| acc * 2 + if *cur { 1 } else { 0 })
    }

    fn convert_button_to_number(&self, button_schematics: &Vec<Vec<usize>>) -> Vec<i32> {
        button_schematics
            .iter()
            .map(|buttons| {
                buttons
                    .iter()
                    .fold(0, |acc, cur| acc + 2i32.pow(*cur as u32))
            })
            .collect()
    }

    fn xor_combination(
        &self,
        buttons: &Vec<i32>,
        target: i32,
        mut res: i32,
        combination: &mut Vec<i32>,
    ) -> usize {
        if res == target {
            return combination.len();
        }
        if buttons.is_empty() {
            return usize::MAX;
        }

        let mut min_length = usize::MAX;
        for idx in 0..buttons.len() {
            combination.push(buttons[idx]);
            res ^= buttons[idx];

            min_length = self
                .xor_combination(&buttons[idx + 1..].to_vec(), target, res, combination)
                .min(min_length);

            res ^= buttons[idx];
            combination.pop();
        }

        min_length
    }

    fn convert_button_to_joltages_counter(
        &self,
        button_schematics: &Vec<Vec<usize>>,
        joltages_number: usize,
    ) -> Vec<Vec<i32>> {
        button_schematics
            .iter()
            .map(|button| {
                (0..joltages_number)
                    .map(|idx| if button.contains(&idx) { 1 } else { 0 })
                    .collect()
            })
            .collect()
    }
}

impl Puzzle for Day10 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let (_, input) = parser::parse(input).unwrap();

        input
            .iter()
            .map(|machine| {
                let lights = self.convert_light_to_number(&machine.indicator_lights);
                let buttons = self.convert_button_to_number(&machine.button_schematics);

                self.xor_combination(&buttons, lights, 0, &mut vec![]) as i64

                // println!("lights: {:?}", lights);
                // println!("buttons: {:?}", buttons);

                // let mut combination = vec![];

                // let res = self.xor_combination(&buttons, lights, 0, &mut combination) as i64;
                // println!("res: {}", res);

                // res
            })
            .sum()
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (_, input) = parser::parse(input).unwrap();

        input
            .iter()
            .map(|machine| {
                let joltages = &machine.joltages;
                let buttons = self
                    .convert_button_to_joltages_counter(&machine.button_schematics, joltages.len());

                println!("joltages: {:?}", joltages);
                println!("button_schematics: {:?}", &machine.button_schematics);
                println!("buttons: {:?}", buttons);

                11
            })
            .sum()
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 10 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 10 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_puzzle_day10_parse() {
        let (_, input) = parser::parse(TESTCASE).unwrap();

        let Machine {
            indicator_lights,
            button_schematics,
            joltages,
        } = &input[0];

        assert_eq!(*indicator_lights, vec![false, true, true, false]);
        assert_eq!(
            *button_schematics,
            vec![
                vec![3],
                vec![1, 3],
                vec![2],
                vec![2, 3],
                vec![0, 2],
                vec![0, 1]
            ]
        );
        assert_eq!(*joltages, vec![3, 5, 4, 7])
    }

    #[test]
    fn test_puzzle_day10_part1() {
        let puzzle = Day10;

        assert_eq!(puzzle.part1(TESTCASE), 7);
    }

    #[test]
    fn test_puzzle_day10_part2() {
        let puzzle = Day10;

        assert_eq!(puzzle.part2(TESTCASE), 33);
    }
}
