use std::collections::HashMap;

fn duplicate_and_count(step: u32, cur: u128, memo: &mut HashMap<(u128, u32), u128>) -> u128 {
    if step == 0 {
        return 1
    }
    if let Some(v) = memo.get(&(cur, step)) {
        return *v;
    }

    let mut count_num = 0;
    let mut digit = 0;
    if cur != 0 {
        digit = cur.ilog10() as u128 + 1;
    }

    if cur == 0 {
        count_num += duplicate_and_count(step - 1,1, memo);
    } else if digit % 2 == 0 {
        let half_way = digit / 2;
        let pow_ten = 10_u128.pow(half_way as u32);
        

        let left_half = cur % pow_ten;
        count_num += duplicate_and_count(step - 1, left_half, memo);
        
        let right_half = cur / pow_ten;
        count_num += duplicate_and_count(step - 1, right_half, memo);
    } else {
        let num_u128 = cur * 2024;

        count_num += duplicate_and_count(step - 1, num_u128, memo);
    }

    memo.insert((cur, step), count_num);

    count_num
}

fn part_one(input: String) -> String {
    let mut count_num: u128 = 0;
    let mut memo: HashMap<(u128, u32), u128> = HashMap::new();
    input.split(" ").for_each(|num| {
        count_num += duplicate_and_count(25, num.parse::<u128>().unwrap(), &mut memo);
    });

    format!("{}", count_num)
}

fn part_two(input: String) -> String {
    let mut count_num: u128 = 0;
    input.split(" ").for_each(|num| {
        let mut memo: HashMap<(u128, u32), u128> = HashMap::new();
        count_num += duplicate_and_count(75, num.parse::<u128>().unwrap(), &mut memo);
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
