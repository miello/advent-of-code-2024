use regex::Regex;

fn solve_machine(x1: i64, y1: i64, x2: i64, y2: i64, x: i64, y: i64) -> Option<i64> {
    let a = (x * y2 - x2 * y) / (x1 * y2 - x2 * y1);
    let b = (x * y1 - y * x1) / (x2 * y1 - x1 * y2);

    if a < 0 || b < 0 {
        return None;
    }

    if a * (x1 * y2 - x2 * y1) != (x * y2 - x2 * y) || b * (x2 * y1 - x1 * y2) != (x * y1 - y * x1)
    {
        return None;
    }

    return Some(3 * a + b);
}

fn part_one(input: String) -> String {
    let regex_input = Regex::new(r".*X\+(?<x1>\d+), Y\+(?<y1>\d+)\n.*X\+(?<x2>\d+), Y\+(?<y2>\d+)\n.*X\=(?<x>\d+)\, Y\=(?<y>\d+)").unwrap();
    let capture_input = regex_input.captures_iter(&input).map(|m| m.extract());
    let mut answer = 0;

    for (_, [x1, y1, x2, y2, x, y]) in capture_input {
        let x1 = x1.parse::<i64>().unwrap();
        let y1 = y1.parse::<i64>().unwrap();
        let x2 = x2.parse::<i64>().unwrap();
        let y2 = y2.parse::<i64>().unwrap();
        let x = x.parse::<i64>().unwrap();
        let y = y.parse::<i64>().unwrap();

        if let Some(result) = solve_machine(x1, y1, x2, y2, x, y) {
            answer += result;
        }
    }

    format!("{}", answer)
}

fn part_two(input: String) -> String {
    let regex_input = Regex::new(r".*X\+(?<x1>\d+), Y\+(?<y1>\d+)\n.*X\+(?<x2>\d+), Y\+(?<y2>\d+)\n.*X\=(?<x>\d+)\, Y\=(?<y>\d+)").unwrap();
    let capture_input = regex_input.captures_iter(&input).map(|m| m.extract());
    let mut answer = 0;

    for (_, [x1, y1, x2, y2, x, y]) in capture_input {
        let x1 = x1.parse::<i64>().unwrap();
        let y1 = y1.parse::<i64>().unwrap();
        let x2 = x2.parse::<i64>().unwrap();
        let y2 = y2.parse::<i64>().unwrap();
        let x = x.parse::<i64>().unwrap() + 10_i64.pow(13);
        let y = y.parse::<i64>().unwrap() + 10_i64.pow(13);

        if let Some(result) = solve_machine(x1, y1, x2, y2, x, y) {
            answer += result;
        }
    }

    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day13_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (
            fs::read_to_string(path_in).expect("Unable to read input"),
            fs::read_to_string(path_out).expect("Unable to read output"),
        )
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day13/part1/sample_in.txt",
            "testcase/day13/part1/sample_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase(
            "testcase/day13/part1/real_in.txt",
            "testcase/day13/part1/real_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day13/part2/sample_in.txt",
            "testcase/day13/part2/sample_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase(
            "testcase/day13/part2/real_in.txt",
            "testcase/day13/part2/real_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }
}
