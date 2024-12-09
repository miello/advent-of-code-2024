use std::collections::{HashMap, HashSet};

fn read_input_part1(input: &str) -> Vec<i32> {
    let mut is_empty_chunk = false;
    let mut num = 0;
    let mut disk_space: Vec<i32> = Vec::new();

    input.split("").filter(|f| !f.eq(&"")).for_each(|ch| {
        let chunk_size = ch.parse::<usize>().unwrap();
        for _ in 0..chunk_size {
            disk_space.push(match is_empty_chunk {
                true => -1,
                false => num,
            });
        }
        if !is_empty_chunk {
            num += 1;
        }
        is_empty_chunk = !is_empty_chunk;
    });

    disk_space
}

fn part_one(input: String) -> String {
    let mut disk_space = read_input_part1(&input);

    let mut left: usize = 0;
    let mut right: usize = disk_space.len() - 1;

    loop {
        while left < disk_space.len() && disk_space[left] != -1 {
            left += 1;
        }
        while right != 0 && disk_space[right] == -1 {
            right -= 1;
        }

        if left > right {
            break;
        }

        disk_space.swap(left, right);
    }

    let mut answer: u64 = 0;

    for i in 0..disk_space.len() {
        if disk_space[i] != -1 {
            answer += (disk_space[i] as u64) * (i as u64);
        }
    }

    format!("{}", answer)
}

fn read_input_part2(input: &str) -> (Vec<(i64, u64)>, i64) {
    let mut is_empty_chunk = false;
    let mut num: i64 = 0;

    (input
        .split("")
        .filter(|f| !f.eq(&""))
        .map(|ch| {
            let chunk_size = ch.parse::<u64>().unwrap();
            let current_num = match is_empty_chunk {
                true => -1,
                false => {
                    let prev_num = num;
                    num += 1;
                    prev_num
                }
            };

            is_empty_chunk = !is_empty_chunk;

            return (current_num, chunk_size);
        })
        .collect::<Vec<_>>(), num)
}

fn part_two(input: String) -> String {
    let (mut disk_space, count_num) = read_input_part2(&input);

    let mut mapping_num: HashMap<i64, u64> = HashMap::new();
    for (num, count) in &disk_space {
        if *num != -1 {
            mapping_num.insert(*num, *count);
        }
    }

    let mut current_num = count_num - 1;
    let mut already_moved: HashSet<i64> = HashSet::new();

    while current_num >= 0 {
        let mut temp_disk_space = disk_space.clone();
        let required_size = *mapping_num.get(&current_num).unwrap();

        for i in 0..disk_space.len() {
            let (num, size) = disk_space[i];
            if num >= current_num && !already_moved.contains(&num) {
                break;
            }
            if num == -1 && required_size <= size {
                temp_disk_space.remove(i);
                already_moved.insert(current_num);

                if size > required_size {
                    temp_disk_space.insert(i, (-1, size - required_size));
                }

                temp_disk_space.insert(i, (current_num, required_size));
                break;
            }
        }

        disk_space = temp_disk_space;
        current_num -= 1;
    }

    let mut already_found_num: HashSet<i64> = HashSet::new();
    let mut accum_start: u64 = 0;
    let mut answer: u64 = 0;

    for (num, size) in disk_space {
        if already_found_num.contains(&num) {
            accum_start += size;
            continue;
        }

        if num != -1 {
            let step_accum = ((accum_start * 2) + size - 1) * size / 2;

            answer += (num as u64) * step_accum;
            already_found_num.insert(num);
        }
        accum_start += size;
    }

    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day9_test {
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
            "testcase/day9/part1/sample_in.txt",
            "testcase/day9/part1/sample_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase(
            "testcase/day9/part1/real_in.txt",
            "testcase/day9/part1/real_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day9/part2/sample_in.txt",
            "testcase/day9/part2/sample_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase(
            "testcase/day9/part2/real_in.txt",
            "testcase/day9/part2/real_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }
}
