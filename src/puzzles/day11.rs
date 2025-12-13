use std::{collections::HashMap, vec};

use super::Puzzle;

pub struct Day11;

struct Device {
    name: String,
    outputs: Vec<String>,
}
type Input = Vec<Device>;

#[derive(PartialEq, Eq, Hash)]
struct DeviceTreeNode {
    name: String,
    children: Vec<String>,
}

mod parser {
    use nom::IResult;

    use super::{Device, Input};

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::combinator::map(
                nom::sequence::separated_pair(
                    nom::combinator::map(nom::character::complete::alpha1, |s: &str| s.to_string()),
                    nom::bytes::complete::tag(": "),
                    nom::multi::separated_list1(
                        nom::character::complete::char(' '),
                        nom::combinator::map(nom::character::complete::alpha1, |s: &str| {
                            s.to_string()
                        }),
                    ),
                ),
                |(name, outputs)| Device { name, outputs },
            ),
        )(input)
    }
}

impl Day11 {
    fn create_server_rack(&self, devices: &Vec<Device>) -> HashMap<String, DeviceTreeNode> {
        let mut server_rack = HashMap::new();

        for device in devices {
            [vec![device.name.clone()], device.outputs.clone()]
                .concat()
                .iter()
                .for_each(|device_name| {
                    server_rack
                        .entry(device_name.clone())
                        .or_insert(DeviceTreeNode {
                            name: device_name.clone(),
                            children: vec![],
                        });
                });
        }

        for device in devices {
            for output in &device.outputs {
                server_rack.entry(device.name.clone()).and_modify(|node| {
                    if !node.children.contains(output) {
                        node.children.push(output.clone());
                    }
                });
            }
        }

        server_rack
    }

    fn find_path_count(
        &self,
        root: &DeviceTreeNode,
        out: &DeviceTreeNode,
        server_rack: &HashMap<String, DeviceTreeNode>,
        memory: &mut HashMap<String, i64>,
    ) -> i64 {
        if root == out {
            memory.entry(root.name.clone()).or_insert(1);
            return 1;
        }
        if memory.contains_key(&root.name) {
            return *memory.get(&root.name).unwrap();
        }

        let path_sum = root
            .children
            .iter()
            .map(|nodename| {
                let node = server_rack.get(nodename).unwrap();
                self.find_path_count(node, out, server_rack, memory)
            })
            .sum();
        memory.entry(root.name.clone()).or_insert(path_sum);

        path_sum
    }

    fn find_path_count_with_flags(
        &self,
        root: &DeviceTreeNode,
        out: &DeviceTreeNode,
        server_rack: &HashMap<String, DeviceTreeNode>,
        has_dac: bool,
        has_fft: bool,
        memo: &mut HashMap<(String, bool, bool), i64>,
    ) -> i64 {
        let has_dac = has_dac || root.name == "dac";
        let has_fft = has_fft || root.name == "fft";
        if root == out {
            return if has_dac && has_fft { 1 } else { 0 };
        }
        let key = (root.name.clone(), has_dac, has_fft);
        if let Some(&v) = memo.get(&key) {
            return v;
        }
        let mut sum = 0;
        for nodename in &root.children {
            let node = server_rack.get(nodename).unwrap();
            sum += self.find_path_count_with_flags(node, out, server_rack, has_dac, has_fft, memo);
        }
        memo.insert(key, sum);
        sum
    }
}

impl Puzzle for Day11 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let devices = parser::parse(input).unwrap().1;

        let server_rack = self.create_server_rack(&devices);
        let root = server_rack.get("you").unwrap();
        let out = server_rack.get("out").unwrap();

        self.find_path_count(root, out, &server_rack, &mut HashMap::new())
    }

    fn part2(&self, input: &str) -> Self::Output {
        let devices = parser::parse(input).unwrap().1;
        let server_rack = self.create_server_rack(&devices);
        let root = server_rack.get("svr").unwrap();
        let out = server_rack.get("out").unwrap();
        let mut memo = HashMap::new();
        self.find_path_count_with_flags(root, out, &server_rack, false, false, &mut memo)
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 11 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 11 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const TESTCASE2: &'static str = r"svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_puzzle_day11_parse() {
        let (_, input) = parser::parse(TESTCASE).unwrap();

        assert_eq!(input[0].name, "aaa");
        assert_eq!(input[0].outputs, vec!["you", "hhh"]);
    }

    #[test]
    fn test_puzzle_day11_part1() {
        let puzzle = Day11;

        assert_eq!(puzzle.part1(TESTCASE), 5);
    }

    #[test]
    fn test_puzzle_day11_part2() {
        let puzzle = Day11;

        assert_eq!(puzzle.part2(TESTCASE2), 2);
    }
}
