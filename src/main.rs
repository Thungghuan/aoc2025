use aoc2025::puzzles::*;
use clap::{Parser, Subcommand};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client,
};
use std::io::prelude::*;
use std::{error::Error, fs, path::Path};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Day { day: i32 },
    New { day: i32 },
}

async fn get_input(day: i32) -> Result<String, Box<dyn Error>> {
    let cache_dir = Path::new("./puzzle_inputs");
    let cache_path = cache_dir.join(format!("day{}.txt", day));

    if cache_path.exists() {
        let s = std::fs::read_to_string(&cache_path)?;
        return Ok(s);
    }

    let url = format!("https://adventofcode.com/2025/day/{}/input", day);
    let cookies = fs::read_to_string(".cookie")?.trim().to_string();

    let mut headers = HeaderMap::new();
    headers.insert(reqwest::header::COOKIE, HeaderValue::from_str(&cookies)?);

    let client = Client::builder().default_headers(headers).build()?;
    let resp = client.get(&url).send().await?;
    let status = resp.status();
    let body = resp.text().await?;

    if status != 200 {
        return Err(format!("unexpected status {}: {}", status, body).into());
    }

    fs::create_dir_all(&cache_dir)?;
    fs::write(&cache_path, &body)?;

    Ok(body)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Day { day } => {
            let data = get_input(day.clone()).await?;

            match day {
                1 => {
                    let puzzle = day1::Day1;
                    puzzle.solve(&data);
                }
                2 => {
                    let puzzle = day2::Day2;
                    puzzle.solve(&data);
                }
                3 => {
                    let puzzle = day3::Day3;
                    puzzle.solve(&data);
                }
                4 => {
                    let puzzle = day4::Day4;
                    puzzle.solve(&data);
                }
                5 => {
                    let puzzle = day5::Day5;
                    puzzle.solve(&data);
                }
                6 => {
                    let puzzle = day6::Day6;
                    puzzle.solve(&data);
                }
                7 => {
                    let puzzle = day7::Day7;
                    puzzle.solve(&data);
                }
                8 => {
                    let puzzle = day8::Day8;
                    puzzle.solve(&data);
                }
                9 => {
                    let puzzle = day9::Day9;
                    puzzle.solve(&data);
                }
                10 => {
                    let puzzle = day10::Day10;
                    puzzle.solve(&data);
                }
                _ => {
                    println!("Puzzle of day {:#?} not found!", day);
                }
            }
        }
        Commands::New { day } => {
            let solution_path = Path::new("./src/puzzles").join(format!("day{}.rs", day));
            if solution_path.exists() {
                panic!("Solution file {:?} exists", solution_path);
            }
            println!("Path: {:?}", solution_path);
            let mut solution_file: fs::File = fs::File::create(solution_path)?;
            writeln!(solution_file, "use super::Puzzle;\n")?;
            writeln!(solution_file, "pub struct Day{};\n", day)?;
            writeln!(solution_file, "type Input = ();\n")?;
            writeln!(
                solution_file,
                "mod parser {{\n    use nom::IResult;\n\n    use super::Input;\n"
            )?;
            writeln!(
                solution_file,
                "    pub fn parse(input: &str) -> IResult<&str, Input> {{"
            )?;
            writeln!(solution_file, "        Ok((\"\", ()))\n    }}\n}}\n")?;
            writeln!(solution_file, "impl Puzzle for Day{} {{", day)?;
            writeln!(solution_file, "    type Output = i64;\n")?;
            writeln!(
                solution_file,
                "    fn part1(&self, input: &str) -> Self::Output {{\n        0\n    }}\n"
            )?;
            writeln!(
                solution_file,
                "    fn part2(&self, input: &str) -> Self::Output {{\n        0\n    }}\n"
            )?;
            writeln!(solution_file, "    fn solve(&self, input: &str) {{")?;
            writeln!(solution_file, "            let ans1 = self.part1(&input);")?;
            writeln!(
                solution_file,
                "            println!(\"Answer of Day {} Part 1:  {{:#?}}\", ans1);",
                day
            )?;
            writeln!(solution_file, "            let ans2 = self.part2(&input);")?;
            writeln!(
                solution_file,
                "            println!(\"Answer of Day {} Part 2:  {{:#?}}\", ans2);",
                day
            )?;
            writeln!(solution_file, "        }}")?;
            writeln!(solution_file, "}}\n")?;
            writeln!(solution_file, "#[cfg(test)]")?;
            writeln!(solution_file, "mod tests {{")?;
            writeln!(solution_file, "    use super::*;\n")?;
            writeln!(solution_file, "    const TESTCASE: &'static str = r\"\";\n")?;
            writeln!(
                solution_file,
                "    #[test]\n    fn test_puzzle_day{}_parse() {{\n\n    }}\n",
                day
            )?;
            writeln!(
                solution_file,
                "    #[test]\n    fn test_puzzle_day{}_part1() {{",
                day
            )?;
            writeln!(solution_file, "        let puzzle = Day{};\n", day)?;
            writeln!(
                solution_file,
                "        assert_eq!(puzzle.part1(TESTCASE), 0);"
            )?;
            writeln!(solution_file, "    }}\n")?;
            writeln!(
                solution_file,
                "    #[test]\n    fn test_puzzle_day{}_part2() {{",
                day
            )?;
            writeln!(solution_file, "        let puzzle = Day{};\n", day)?;
            writeln!(
                solution_file,
                "        assert_eq!(puzzle.part2(TESTCASE), 0);"
            )?;
            writeln!(solution_file, "    }}\n")?;
            writeln!(solution_file, "}}")?;

            let all_mod_line = fs::read_to_string(Path::new("./src/puzzles").join("mod.rs"))?;
            let mut all_mod_lines: Vec<String> = all_mod_line
                .trim()
                .split("\n")
                .map(|line| line.to_string())
                .collect::<Vec<String>>();

            let insert_idx = all_mod_lines
                .iter()
                .filter(|&line| line.starts_with("pub mod day"))
                .count();

            let new_mod_line = format!("pub mod day{};", day);
            if !all_mod_lines.contains(&new_mod_line) {
                all_mod_lines.insert(insert_idx, new_mod_line);
            }

            let mut solution_mod_file = fs::OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(Path::new("./src/puzzles").join("mod.rs"))?;

            writeln!(solution_mod_file, "{}", all_mod_lines.join("\n"))?;

            let main_path = Path::new("./src/main.rs");
            let main_text = fs::read_to_string(main_path)?;

            if !main_text.contains(&format!("day{}::Day{}", day, day)) {
                let mut main_lines: Vec<String> =
                    main_text.lines().map(|l| l.to_string()).collect();

                let insert_idx = main_lines
                    .iter()
                    .position(|l| l.contains("_ => {"))
                    .unwrap_or(main_lines.len());

                let new_arm = format!(
                    "                {d} => {{\n                    let puzzle = day{d}::Day{d};\n                    puzzle.solve(&data);\n                }}",
                    d = day
                );

                main_lines.insert(insert_idx, new_arm);
                fs::write(main_path, main_lines.join("\n"))?;
            }
        }
    };

    Ok(())
}