use super::Puzzle;
use std::io::Write;

pub struct Day9;

type Position = (i64, i64);
type Input = Vec<Position>;

mod parser {
    use nom::IResult;

    use super::Input;

    pub fn parse(input: &str) -> IResult<&str, Input> {
        nom::multi::separated_list1(
            nom::character::complete::newline,
            nom::sequence::separated_pair(
                nom::character::complete::i64,
                nom::bytes::complete::tag(","),
                nom::character::complete::i64,
            ),
        )(input)
    }
}

impl Day9 {
    fn count_area(&self, point_a: Position, point_b: Position) -> i64 {
        ((point_a.0 - point_b.0).abs() + 1) * ((point_a.1 - point_b.1).abs() + 1)
    }

    fn generate_floor(&self, tiles: &Vec<Position>) -> Vec<Vec<bool>> {
        let mut max_row = 0;
        let mut max_col = 0;
        for tile in tiles {
            max_row = max_row.max(tile.1);
            max_col = max_col.max(tile.0)
        }
        let floor_row = max_row as usize + 2;
        let floor_col = max_col as usize + 2;

        vec![vec![false; floor_col]; floor_row]
    }

    fn add_border(&self, floor: &mut Vec<Vec<bool>>, tiles: &Vec<Position>) {
        for i in 0..tiles.len() {
            let (x1, y1) = tiles[i];
            let (x2, y2) = tiles[(i + 1) % tiles.len()];

            let xmin = x1.min(x2) as usize;
            let xmax = x1.max(x2) as usize;
            let ymin = y1.min(y2) as usize;
            let ymax = y1.max(y2) as usize;

            if x1 == x2 {
                for row in ymin..=ymax {
                    floor[row][xmin] = true;
                }
            } else {
                for col in xmin..=xmax {
                    floor[ymin][col] = true;
                }
            }
        }
    }

    fn fill_tiles(&self, floor: &mut Vec<Vec<bool>>, tiles: &Vec<Position>) {
        // Integer scanline fill: for each integer row y, compute intersections of
        // polygon edges with the horizontal line y, include horizontal edges that
        // lie on that y (push both endpoints), sort intersections and fill between
        // pairs [x0,x1], [x2,x3], ...
        let rows = floor.len();
        if rows == 0 {
            return;
        }
        let cols = floor[0].len();

        for row in 0..rows {
            let y = row as i64;
            let mut xs: Vec<f64> = Vec::new();

            for i in 0..tiles.len() {
                let (x1, y1) = tiles[i];
                let (x2, y2) = tiles[(i + 1) % tiles.len()];

                // horizontal edge on this row: include both endpoints
                if y1 == y2 {
                    if y1 == y {
                        xs.push(x1 as f64);
                        xs.push(x2 as f64);
                    }
                    continue;
                }

                // for non-horizontal edges, include if y in [min(y1,y2), max(y1,y2))
                let ymin = y1.min(y2);
                let ymax = y1.max(y2);
                if y >= ymin && y < ymax {
                    let xinters = x1 as f64
                        + (y as f64 - y1 as f64) * (x2 as f64 - x1 as f64)
                            / (y2 as f64 - y1 as f64);
                    xs.push(xinters);
                }
            }

            if xs.is_empty() {
                continue;
            }

            xs.sort_by(|a, b| a.partial_cmp(b).unwrap());

            let mut k = 0;
            while k + 1 < xs.len() {
                let xl = xs[k];
                let xr = xs[k + 1];

                let start_col = xl.ceil() as isize;
                let end_col = xr.floor() as isize;
                if start_col <= end_col {
                    let start = start_col.max(0) as usize;
                    let end = end_col.min((cols as isize) - 1).max(0) as usize;
                    if start <= end {
                        for c in start..=end {
                            floor[row][c] = true;
                        }
                    }
                }

                k += 2;
            }
        }
    }

    fn visualize_floor(&self, floor: &Vec<Vec<bool>>) {
        let outputs_path = std::path::Path::new("./puzzle_inputs").join("day9_output.txt");
        let mut output_file = std::fs::File::create(outputs_path).unwrap();

        for row in floor {
            let mut row_str = String::new();
            for tile in row {
                if *tile {
                    row_str += "#";
                } else {
                    row_str += ".";
                }
            }
            // println!("{}", row_str);

            writeln!(output_file, "{:}", row_str).unwrap();
        }
    }
}

impl Puzzle for Day9 {
    type Output = i64;

    fn part1(&self, input: &str) -> Self::Output {
        let (_, tiles) = parser::parse(input).unwrap();
        let mut max_area = 0;

        for i in 0..tiles.len() {
            for j in 0..i {
                max_area = max_area.max(self.count_area(tiles[i], tiles[j]));
            }
        }

        max_area
    }

    fn part2(&self, input: &str) -> Self::Output {
        let (_, tiles) = parser::parse(input).unwrap();
        let mut floor = self.generate_floor(&tiles);
        self.add_border(&mut floor, &tiles);
        self.fill_tiles(&mut floor, &tiles);
        self.visualize_floor(&floor);

        let mut max_area = 0;
        for i in 0..tiles.len() {
            for j in 0..i {
                let tile_i = tiles[i];
                let tile_j = tiles[j];

                let xmin = tile_i.0.min(tile_j.0) as usize;
                let ymin = tile_i.1.min(tile_j.1) as usize;
                let xmax = tile_i.0.max(tile_j.0) as usize;
                let ymax = tile_i.1.max(tile_j.1) as usize;

                let mut is_valid = true;
                for row in ymin..=ymax {
                    for col in xmin..=xmax {
                        if !floor[row][col] {
                            is_valid = false;
                            break;
                        }
                    }

                    if !is_valid {
                        break;
                    }
                }

                if is_valid {
                    max_area = max_area.max(self.count_area(tiles[i], tiles[j]));
                }
            }
        }

        max_area
    }

    fn solve(&self, input: &str) {
        let ans1 = self.part1(&input);
        println!("Answer of Day 9 Part 1:  {:#?}", ans1);
        let ans2 = self.part2(&input);
        println!("Answer of Day 9 Part 2:  {:#?}", ans2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TESTCASE: &'static str = r"7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

    const _TESTCASE2: &'static str = r"7,1
11,1
11,7
9,7
9,5
7,5
7,7
4,7
4,5
2,5
2,3
7,3";

    #[test]
    fn test_puzzle_day9_parse() {
        let (_, input) = parser::parse(TESTCASE).unwrap();

        assert_eq!(
            input,
            vec![
                (7, 1),
                (11, 1),
                (11, 7),
                (9, 7),
                (9, 5),
                (2, 5),
                (2, 3),
                (7, 3),
            ]
        );
    }

    #[test]
    fn test_puzzle_day9_part1() {
        let puzzle = Day9;

        assert_eq!(puzzle.part1(TESTCASE), 50);
    }

    #[test]
    fn test_puzzle_day9_part2() {
        let puzzle = Day9;

        assert_eq!(puzzle.part2(TESTCASE), 24);
    }
}
