use regex::Regex;

fn part_one(input: String) -> String {
    let re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();
    let capture_input = re.captures_iter(&input).map(|m| m.extract());
    let mut answer = 0;

    for (_, [a, b]) in capture_input {
        let a_int = str::parse::<i32>(a).unwrap();
        let b_int = str::parse::<i32>(b).unwrap();

        answer += a_int * b_int;
    }


    format!("{}", answer)
}

fn part_two(input: String) -> String {
    let re = Regex::new(r"(mul\(\d+,\d+\))|(don\'t\(\))|(do\(\))").unwrap();
    let mul_re = Regex::new(r"mul\((?<a>\d+),(?<b>\d+)\)").unwrap();

    let capture_input = re.captures_iter(&input).map(|m| m.extract());
    let mut answer = 0;
    let mut enabled = true;

    for (_, [a]) in capture_input {
        if a == "don\'t()" {
            enabled = false;
        } else if a == "do()" {
            enabled = true;
        } else if enabled {
            let result = mul_re.captures(a).unwrap();
            let a = result.get(1).unwrap().as_str();
            let b = result.get(2).unwrap().as_str();

            let a_int = str::parse::<i32>(a).unwrap();
            let b_int = str::parse::<i32>(b).unwrap();
    
            answer += a_int * b_int;
        }
    }

    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day3_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase("testcase/day3/part1/sample_in.txt", "testcase/day3/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase("testcase/day3/part1/real_in.txt", "testcase/day3/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase("testcase/day3/part2/sample_in.txt", "testcase/day3/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase("testcase/day3/part2/real_in.txt", "testcase/day3/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }
}
