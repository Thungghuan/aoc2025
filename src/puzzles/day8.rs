use std::{collections::HashSet, usize};

use super::Puzzle;

pub struct Day8;

type Position = (i64, i64, i64);
type Input = Vec<Position>;

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::combinator::map_res(
            nom::multi::separated_list1(
                nom::character::complete::newline,
                nom::multi::separated_list1(
                    nom::bytes::complete::tag(","),
                    nom::character::complete::i64,
                ),
            ),
            |res| Ok::<Input, &str>(res.iter().map(|pos| (pos[0], pos[1], pos[2])).collect()),
        )(input)
    }
}

impl Day8 {
    fn calc_dist(&self, point_a: Position, point_b: Position) -> f64 {
        let (a_x, a_y, a_z) = point_a;
        let (b_x, b_y, b_z) = point_b;

        (((a_x - b_x).pow(2) + (a_y - b_y).pow(2) + (a_z - b_z).pow(2)) as f64).sqrt()
    }
}

impl Puzzle for Day8 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let (_, junctions) = parser::parse(input).unwrap();
        let mut all_dist = vec![];
        for i in 0..junctions.len() {
            for j in 0..i {
                all_dist.push((self.calc_dist(junctions[i], junctions[j]), (i, j)));
            }
        }

        let times = {
            #[cfg(test)]
            {
                10
            }

            #[cfg(not(test))]
            {
                1000
            }
        };

        all_dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut junction_sets: Vec<HashSet<usize>> = vec![];
        for idx in 0..times {
            let (_dist, (junction_a, junction_b)) = all_dist[idx];

            let junction_a_idx = match junction_sets
                .iter()
                .position(|junctions| junctions.contains(&junction_a))
            {
                Some(index) => index,
                None => {
                    junction_sets.push(HashSet::from([junction_a]));
                    junction_sets.len() - 1
                }
            };

            match junction_sets
                .iter()
                .position(|junctions| junctions.contains(&junction_b))
            {
                Some(junction_b_idx) => {
                    if junction_b_idx > junction_a_idx {
                        let junction_b_set = junction_sets.remove(junction_b_idx);
                        junction_sets[junction_a_idx].extend(junction_b_set);
                    } else if junction_b_idx < junction_a_idx {
                        let junction_a_set = junction_sets.remove(junction_a_idx);
                        junction_sets[junction_b_idx].extend(junction_a_set);
                    }
                }
                None => {
                    junction_sets[junction_a_idx].insert(junction_b);
                }
            };
        }

        let mut junction_sets_count = junction_sets
            .iter()
            .map(|set| set.len() as i64)
            .collect::<Vec<i64>>();
        junction_sets_count.sort_by(|a, b| b.cmp(a));

        junction_sets_count[0..3].iter().product()
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (_, junctions) = parser::parse(input).unwrap();
        let jucntion_count = junctions.len();
        let mut conn_count = 0;

        let mut last_junction_a = 0;
        let mut last_junction_b = 0;

        let mut all_dist = vec![];
        for i in 0..junctions.len() {
            for j in 0..i {
                all_dist.push((self.calc_dist(junctions[i], junctions[j]), (i, j)));
            }
        }

        all_dist.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        let mut junction_sets: Vec<HashSet<usize>> = vec![];
        for idx in 0..all_dist.len() {
            let (_dist, (junction_a, junction_b)) = all_dist[idx];

            let junction_a_idx = match junction_sets
                .iter()
                .position(|junctions| junctions.contains(&junction_a))
            {
                Some(index) => index,
                None => {
                    junction_sets.push(HashSet::from([junction_a]));
                    conn_count += 1;
                    junction_sets.len() - 1
                }
            };

            match junction_sets
                .iter()
                .position(|junctions| junctions.contains(&junction_b))
            {
                Some(junction_b_idx) => {
                    if junction_b_idx > junction_a_idx {
                        let junction_b_set = junction_sets.remove(junction_b_idx);
                        junction_sets[junction_a_idx].extend(junction_b_set);
                    } else if junction_b_idx < junction_a_idx {
                        let junction_a_set = junction_sets.remove(junction_a_idx);
                        junction_sets[junction_b_idx].extend(junction_a_set);
                    }
                }
                None => {
                    conn_count += 1;
                    junction_sets[junction_a_idx].insert(junction_b);
                }
            };

            if conn_count == jucntion_count && junction_sets.len() == 1 {
                last_junction_a = junction_a;
                last_junction_b = junction_b;
                break;
            }
        }

        junctions[last_junction_a].0 * junctions[last_junction_b].0
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 8 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 8 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

    #[test]
    fn test_puzzle_day8_parse() {
        let (_, input) = parser::parse(TESTCASE).unwrap();

        assert_eq!(
            input,
            vec![
                (162, 817, 812),
                (57, 618, 57),
                (906, 360, 560),
                (592, 479, 940),
                (352, 342, 300),
                (466, 668, 158),
                (542, 29, 236),
                (431, 825, 988),
                (739, 650, 466),
                (52, 470, 668),
                (216, 146, 977),
                (819, 987, 18),
                (117, 168, 530),
                (805, 96, 715),
                (346, 949, 466),
                (970, 615, 88),
                (941, 993, 340),
                (862, 61, 35),
                (984, 92, 344),
                (425, 690, 689),
            ]
        )
    }

    #[test]
    fn test_puzzle_day8_part1() {
        let puzzle = Day8;

        assert_eq!(puzzle.part1(TESTCASE), 40);
    }

    #[test]
    fn test_puzzle_day8_part2() {
        let puzzle = Day8;

        assert_eq!(puzzle.part2(TESTCASE), 25272);
    }
}
