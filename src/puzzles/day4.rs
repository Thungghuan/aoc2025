use super::Puzzle;

pub struct Day4;

type Input = Vec<Vec<char>>;

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::multi::many1(nom::character::complete::satisfy(|ch| {
                ch.is_ascii_graphic()
            })),
        )(input)
    }
}

impl Day4 {
    fn find_remove_pos(&self, grid: &Input) -> Option<Vec<(usize, usize)>> {
        let mut to_remove = vec![];
        let m = grid.len();
        let n = grid[0].len();

        grid.iter().enumerate().for_each(|(i, row)| {
            row.iter().enumerate().for_each(|(j, col)| {
                let mut adjacents = std::collections::HashSet::new();
                if i > 0 {
                    adjacents.insert((i - 1, j));
                    if j > 0 {
                        adjacents.insert((i - 1, j - 1));
                        adjacents.insert((i, j - 1));
                    }
                    if j < n - 1 {
                        adjacents.insert((i - 1, j + 1));
                        adjacents.insert((i, j + 1));
                    }
                }
                if i < m - 1 {
                    adjacents.insert((i + 1, j));
                    if j > 0 {
                        adjacents.insert((i + 1, j - 1));
                        adjacents.insert((i, j - 1));
                    }
                    if j < n - 1 {
                        adjacents.insert((i + 1, j + 1));
                        adjacents.insert((i, j + 1));
                    }
                }
                if *col == '@'
                    && adjacents
                        .iter()
                        .filter(|(i, j)| grid[*i][*j] == '@')
                        .count()
                        < 4
                {
                    to_remove.push((i, j));
                }
            })
        });

        if to_remove.len() > 0 {
            Some(to_remove)
        } else {
            None
        }
    }
}

impl Puzzle for Day4 {
    type Output = usize;

    fn part1(&self, input: &str) -> Self::Output {
        let (_, grid) = parser::parse(input).unwrap();

        self.find_remove_pos(&grid).unwrap().len()
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (_, mut grid) = parser::parse(input).unwrap();
        let mut remove_count = 0;

        while let Some(to_remove) = self.find_remove_pos(&grid) {
            remove_count += to_remove.len();

            to_remove.iter().for_each(|&(i, j)| {
                grid[i][j] = '.';
            })
        }

        remove_count
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 4 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 4 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_puzzle_day4_parse() {
        let input = parser::parse(TESTCASE).unwrap().1;
        assert_eq!(
            input,
            vec![
                vec!['.', '.', '@', '@', '.', '@', '@', '@', '@', '.'],
                vec!['@', '@', '@', '.', '@', '.', '@', '.', '@', '@'],
                vec!['@', '@', '@', '@', '@', '.', '@', '.', '@', '@'],
                vec!['@', '.', '@', '@', '@', '@', '.', '.', '@', '.'],
                vec!['@', '@', '.', '@', '@', '@', '@', '.', '@', '@'],
                vec!['.', '@', '@', '@', '@', '@', '@', '@', '.', '@'],
                vec!['.', '@', '.', '@', '.', '@', '.', '@', '@', '@'],
                vec!['@', '.', '@', '@', '@', '.', '@', '@', '@', '@'],
                vec!['.', '@', '@', '@', '@', '@', '@', '@', '@', '.'],
                vec!['@', '.', '@', '.', '@', '@', '@', '.', '@', '.']
            ]
        )
    }

    #[test]
    fn test_puzzle_day4_part1() {
        let puzzle = Day4;
        assert_eq!(puzzle.part1(TESTCASE), 13);
    }

    #[test]
    fn test_puzzle_day4_part2() {
        let puzzle = Day4;
        assert_eq!(puzzle.part2(TESTCASE), 43);
    }
}
