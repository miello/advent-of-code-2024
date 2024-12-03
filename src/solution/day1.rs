use std::{collections::HashMap, iter::zip};

fn part_one(input: String) -> String {
    let mut x_pos: Vec<i32> = Vec::new();
    let mut y_pos: Vec<i32> = Vec::new();

    input.lines().for_each(|line| {
        let line_trim = line.trim();
        if line_trim == "" {
            return;
        }
        let (x, y) = line.split_once("   ").unwrap();

        x_pos.push(x.parse::<i32>().unwrap());
        y_pos.push(y.parse::<i32>().unwrap());
    });

    x_pos.sort_by(|a, b| a.cmp(b));
    y_pos.sort_by(|a, b| a.cmp(b));

    let mut distance: i32 = 0;

    zip(x_pos, y_pos).for_each(|(a, b)| {
        distance += (a - b).abs();
    });

    format!("{}", distance)
}

fn part_two(input: String) -> String {
    let mut x_pos: Vec<i32> = Vec::new();
    let mut y_count: HashMap<i32, i32> = HashMap::new();

    input.lines().for_each(|line| {
        let line_trim = line.trim();
        if line_trim == "" {
            return;
        }

        let (x, y) = line.split_once("   ").unwrap();

        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();

        x_pos.push(x);

        if let Some(x) = y_count.get_mut(&y) {
            *x += 1;
        } else {
            y_count.insert(y, 1);
        }
    });

    let mut similarity = 0;

    x_pos.iter().for_each(|a| {
        if let Some(x) = y_count.get(a) {
            similarity += x * a;
        }
    });

    format!("{}", similarity)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day1_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase("testcase/day1/part1/sample_in.txt", "testcase/day1/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase("testcase/day1/part1/real_in.txt", "testcase/day1/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase("testcase/day1/part2/sample_in.txt", "testcase/day1/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase("testcase/day1/part2/real_in.txt", "testcase/day1/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }
}