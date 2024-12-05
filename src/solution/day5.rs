use std::{cmp::Ordering, collections::HashSet};


fn part_one(input: String) -> String {
    let mut is_page_order = true;
    let mut page_order_list: HashSet<(i32, i32)> = HashSet::new();

    let mut answer = 0;

    input.lines().for_each(|line| {
        if line.trim() == "" {
            is_page_order = false;
            return;
        }

        if is_page_order {
            let (before, after) = line.trim().split_once("|").unwrap();

            let before_int = before.parse::<i32>().unwrap();
            let after_int = after.parse::<i32>().unwrap();

            page_order_list.insert((after_int, before_int));
        }

        if !is_page_order {
            let mut input_page: Vec<_> = line.trim().split(",").map(|page| {
                page.parse::<i32>().unwrap()
            }).collect();

            let mut valid = true;

            input_page.sort_by(|a, b| {
                return match page_order_list.get(&(*b, *a)) {
                    Some(_) => {
                        valid = false;
                        Ordering::Less
                    },
                    None => Ordering::Equal,
                }
            });
            
            if valid {
                answer += input_page[(input_page.len() - 1) / 2];
            }
        }
    });

    format!("{}", answer)
}

fn part_two(input: String) -> String {
    let mut is_page_order = true;
    let mut page_order_list: HashSet<(i32, i32)> = HashSet::new();

    let mut answer = 0;

    input.lines().for_each(|line| {
        if line.trim() == "" {
            is_page_order = false;
            return;
        }

        if is_page_order {
            let (before, after) = line.trim().split_once("|").unwrap();

            let before_int = before.parse::<i32>().unwrap();
            let after_int = after.parse::<i32>().unwrap();

            page_order_list.insert((after_int, before_int));
        }

        if !is_page_order {
            let mut input_page: Vec<_> = line.trim().split(",").map(|page| {
                page.parse::<i32>().unwrap()
            }).collect();

            let mut valid = true;

            input_page.sort_by(|a, b| {
                return match page_order_list.get(&(*b, *a)) {
                    Some(_) => {
                        valid = false;
                        Ordering::Less
                    },
                    None => Ordering::Equal,
                }
            });
            
            if !valid {
                answer += input_page[(input_page.len() - 1) / 2];
            }
        }
    });

    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day5_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase("testcase/day5/part1/sample_in.txt", "testcase/day5/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase("testcase/day5/part1/real_in.txt", "testcase/day5/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase("testcase/day5/part2/sample_in.txt", "testcase/day5/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase("testcase/day5/part2/real_in.txt", "testcase/day5/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }
}
