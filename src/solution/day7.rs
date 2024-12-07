fn is_valid(
    number_input: &Vec<i64>,
    cur: i64,
    limit: i64,
    cur_idx: usize,
    concat_allow: bool,
) -> bool {
    if cur > limit {
        return false;
    }
    if cur_idx == number_input.len() {
        return cur == limit;
    }

    let num_digit = number_input[cur_idx].to_string().len();
    let new_num = cur * 10_i64.pow(num_digit as u32) + number_input[cur_idx];

    if concat_allow && is_valid(number_input, new_num, limit, cur_idx + 1, concat_allow) {
        return true;
    }

    if is_valid(
        number_input,
        cur * number_input[cur_idx],
        limit,
        cur_idx + 1,
        concat_allow,
    ) {
        return true;
    }

    return is_valid(
        number_input,
        cur + number_input[cur_idx],
        limit,
        cur_idx + 1,
        concat_allow,
    );
}

fn part_one(input: String) -> String {
    let mut answer: i64 = 0;

    input.lines().for_each(|line| {
        let (expected_str, input_number_str) = line.split_once(": ").unwrap();

        let number_input: Vec<_> = input_number_str
            .split(" ")
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        let expected_i64 = expected_str.parse::<i64>().unwrap();

        if is_valid(&number_input, number_input[0], expected_i64, 1, false) {
            answer += expected_i64;
        }
    });

    format!("{}", answer)
}

fn part_two(input: String) -> String {
    let mut answer: i64 = 0;

    input.lines().for_each(|line| {
        let (expected_str, input_number_str) = line.split_once(": ").unwrap();

        let number_input: Vec<_> = input_number_str
            .split(" ")
            .map(|num| num.parse::<i64>().unwrap())
            .collect();
        let expected_i64 = expected_str.parse::<i64>().unwrap();

        if is_valid(&number_input, number_input[0], expected_i64, 1, true) {
            answer += expected_i64;
        }
    });

    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day7_test {
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
            "testcase/day7/part1/sample_in.txt",
            "testcase/day7/part1/sample_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase(
            "testcase/day7/part1/real_in.txt",
            "testcase/day7/part1/real_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day7/part2/sample_in.txt",
            "testcase/day7/part2/sample_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase(
            "testcase/day7/part2/real_in.txt",
            "testcase/day7/part2/real_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }
}
