use std::collections::{HashMap, HashSet};

fn read_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|f| !f.eq(&""))
                .collect()
        })
        .collect()
}

fn is_pos_valid(x: i32, y: i32, max_x: i32, max_y: i32) -> bool {
    return 0 <= x && x < max_x && 0 <= y && y < max_y;
}

fn part_one(input: String) -> String {
    let board = read_input(&input);
    let mut antennas_pos: HashMap<&str, Vec<(i32, i32)>> = HashMap::new();

    let max_x = board.len();
    let max_y = board[0].len();

    for i in 0..max_x {
        for j in 0..max_y {
            if board[i][j] != "." {
                if let Some(v) = antennas_pos.get_mut(&board[i][j]) {
                    v.push((i as i32, j as i32));
                } else {
                    antennas_pos.insert(&board[i][j], vec![(i as i32, j as i32)]);
                }
            }
        }
    }

    let mut antinode_pos: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennas) in antennas_pos {
        let antenna_count = antennas.len();
        for i in 0..antenna_count {
            for j in i + 1..antenna_count {
                let diff_x = antennas[j].0 - antennas[i].0;
                let diff_y = antennas[j].1 - antennas[i].1;

                if is_pos_valid(antennas[j].0 + diff_x, antennas[j].1 + diff_y, max_x as i32, max_y as i32) {
                    antinode_pos.insert((antennas[j].0 + diff_x, antennas[j].1 + diff_y));
                }

                if is_pos_valid(antennas[i].0 - diff_x, antennas[i].1 - diff_y, max_x as i32, max_y as i32) {
                    antinode_pos.insert((antennas[i].0 - diff_x, antennas[i].1 - diff_y));
                }
            }
        }
    }

    format!("{}", antinode_pos.len())
}

fn part_two(input: String) -> String {
    let board = read_input(&input);
    let mut antennas_pos: HashMap<&str, Vec<(i32, i32)>> = HashMap::new();

    let max_x = board.len();
    let max_y = board[0].len();

    for i in 0..max_x {
        for j in 0..max_y {
            if board[i][j] != "." {
                if let Some(v) = antennas_pos.get_mut(&board[i][j]) {
                    v.push((i as i32, j as i32));
                } else {
                    antennas_pos.insert(&board[i][j], vec![(i as i32, j as i32)]);
                }
            }
        }
    }

    let mut antinode_pos: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennas) in antennas_pos {
        let antenna_count = antennas.len();
        for i in 0..antenna_count {
            for j in i + 1..antenna_count {
                let diff_x = antennas[j].0 - antennas[i].0;
                let diff_y = antennas[j].1 - antennas[i].1;

                let mut step = 0;
                loop {
                    let new_x = antennas[j].0 + (diff_x * step);
                    let new_y = antennas[j].1 + (diff_y * step);
                    if is_pos_valid(new_x, new_y, max_x as i32, max_y as i32) {
                        antinode_pos.insert((new_x, new_y));
                    } else {
                        break;
                    }
                    step += 1;
                }

                let mut step = 0;
                loop {
                    let new_x = antennas[i].0 - (diff_x * step);
                    let new_y = antennas[i].1 - (diff_y * step);
                    if is_pos_valid(new_x, new_y, max_x as i32, max_y as i32) {
                        antinode_pos.insert((new_x, new_y));
                    } else {
                        break;
                    }
                    step += 1;
                }
            }
        }
    }

    format!("{}", antinode_pos.len())
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day8_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase("testcase/day8/part1/sample_in.txt", "testcase/day8/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase("testcase/day8/part1/real_in.txt", "testcase/day8/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase("testcase/day8/part2/sample_in.txt", "testcase/day8/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase("testcase/day8/part2/real_in.txt", "testcase/day8/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }
}
