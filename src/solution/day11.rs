use std::collections::HashMap;

fn duplicate_and_count(step: u32, cur: &str, memo: &mut HashMap<(String, u32), u64>) -> u64 {
    if step == 0 {
        return 1
    }
    if let Some(v) = memo.get(&(cur.to_string(), step)) {
        return *v;
    }

    let mut count_num = 0;

    if cur == "0" {
        count_num += duplicate_and_count(step - 1, "1", memo);
    } else if cur.len() % 2 == 0 {
        let half_way = cur.len() / 2;

        let left_half = cur[0..half_way].parse::<u128>().unwrap().to_string();
        count_num += duplicate_and_count(step - 1, &left_half, memo);
        
        let right_half = cur[half_way..cur.len()].parse::<u128>().unwrap().to_string();
        count_num += duplicate_and_count(step - 1, &right_half, memo);
    } else {
        let num_u64 = cur.parse::<u128>().unwrap() * 2024;

        count_num += duplicate_and_count(step - 1, &num_u64.to_string(), memo);
    }

    memo.insert((cur.to_string(), step), count_num);

    count_num
}

fn part_one(input: String) -> String {
    let mut count_num: u64 = 0;
    let mut memo: HashMap<(String, u32), u64> = HashMap::new();
    input.split(" ").for_each(|num| {
        count_num += duplicate_and_count(25, num, &mut memo);
    });

    format!("{}", count_num)
}

fn part_two(input: String) -> String {
    let mut count_num: u64 = 0;
    input.split(" ").for_each(|num| {
        let mut memo: HashMap<(String, u32), u64> = HashMap::new();
        count_num += duplicate_and_count(75, num, &mut memo);
    });

    format!("{}", count_num)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day11_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase("testcase/day11/part1/sample_in.txt", "testcase/day11/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase("testcase/day11/part1/real_in.txt", "testcase/day11/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase("testcase/day11/part2/sample_in.txt", "testcase/day11/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase("testcase/day11/part2/real_in.txt", "testcase/day11/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }
}
