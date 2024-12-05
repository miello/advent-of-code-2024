fn is_valid(first: i32, second: i32, state: &str) -> bool {
    let diff = second - first;
    let mut valid = true;
    if 1 > diff.abs() || diff.abs() > 3 {
        valid = false;
    }
    if diff > 0 {
        if state == "decrease" {
            valid = false;
        }
    } else {
        if state == "increase" {
            valid = false
        }
    }
    return valid
}

fn part_one(input: String) -> String {
    let mut count_safe = 0;
    input.lines().for_each(|line| {
        let reports = line.split_whitespace().map(|input| {
            input.parse::<i32>().unwrap()
        });
        let input_vec = reports.collect::<Vec<i32>>();

        let direction;
        let mut valid = true;

        let diff = input_vec[1] - input_vec[0];
        
        if diff > 0 { direction = "increase"; }
        else { direction = "decrease"; }

        for i in 0..input_vec.len() - 1 {
            valid &= is_valid(input_vec[i], input_vec[i + 1], direction);
        }

        if valid {
            count_safe += 1;
        }
    });

    format!("{}", count_safe)
}

fn part_two(input: String) -> String {
    let mut count_safe = 0;
    input.lines().for_each(|line| {
        let reports = line.split_whitespace().map(|input| {
            input.parse::<i32>().unwrap()
        });
        
        let input_vec = reports.collect::<Vec<i32>>();

        let diff = input_vec[1] - input_vec[0];
        let mut direction;
        let mut valid = true;
        
        if diff > 0 { direction = "increase"; }
        else { direction = "decrease"; }

        for i in 0..input_vec.len() - 1 {
            valid &= is_valid(input_vec[i], input_vec[i + 1], direction);
        }

        if valid {
            count_safe += 1;
            return;
        }

        for i in 0..input_vec.len() {
            let mut new_vec = input_vec.clone();
            new_vec.remove(i);
            
            valid = true;

            let diff = new_vec[1] - new_vec[0];
        
            if diff > 0 { direction = "increase"; }
            else { direction = "decrease"; }
    
            for i in 0..new_vec.len() - 1 {
                valid &= is_valid(new_vec[i], new_vec[i + 1], direction);
            }
    
            if valid {
                count_safe += 1;
                return;
            }
        }
    });

    format!("{}", count_safe)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day2_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase("testcase/day2/part1/sample_in.txt", "testcase/day2/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase("testcase/day2/part1/real_in.txt", "testcase/day2/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase("testcase/day2/part2/sample_in.txt", "testcase/day2/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase("testcase/day2/part2/real_in.txt", "testcase/day2/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }
}
