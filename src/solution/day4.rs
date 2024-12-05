fn part_one(input: String) -> String {
    let chr_line = input.lines().map(|line| {
        line.as_bytes().to_vec()
    }).collect::<Vec<_>>();
    
    let diff: Vec<[i32; 2]> = vec![[0, 1], [1, 1], [1, 0], [1, -1], [-1, 1], [-1, -1], [-1, 0], [0, -1]];

    let target = "XMAS".bytes().collect::<Vec<_>>();
    let target_len = target.len();

    let max_x = chr_line.len();
    let max_y = chr_line[0].len();

    let mut answer = 0;

    for i in 0..max_x {
        for j in 0..max_y {
            for k in 0..diff.len() {
                let mut cur = 0;
                while cur < target_len {
                    let new_x = i as i32 + diff[k][0] * (cur as i32);
                    let new_y = j as i32 + diff[k][1] * (cur as i32);
    
                    if 0 <= new_x && new_x < max_x as i32 && 0 <= new_y && new_y < max_y as i32 {
                        if chr_line[new_x as usize][new_y as usize] != target[cur] {
                            break;
                        }
                    } else {
                        break;
                    }
    
                    cur += 1;
                }
    
                if cur == target_len {
                    answer += 1;
                }
            }
        }
    }

    format!("{}", answer)
}

fn part_two(input: String) -> String {
    let chr_line = input.lines().map(|line| {
        line.as_bytes().to_vec()
    }).collect::<Vec<_>>();
    
    let max_x = chr_line.len();
    let max_y = chr_line[0].len();

    let target = "MAS".bytes().collect::<Vec<_>>();
    let mut answer = 0;

    let diff: Vec<[(i32, i32); 2]> = vec![
        [(1, -1), (1, 1)], 
        [(1, -1), (-1, -1)],
        [(-1, 1), (-1, -1)],
        [(-1, 1), (1, 1)],
    ];

    for i in 0..max_x {
        for j in 0..max_y {
            if chr_line[i][j] == target[1] {
                for k in 0..diff.len() {
                    let mut is_success = true;
    
                    for (diff_x, diff_y) in diff[k] {
                        for step in -1..2 {
                            let new_x = i as i32 + diff_x * step;
                            let new_y = j as i32 + diff_y * step;

                            let idx = step + 1;

                            if 0 <= new_x && new_x < max_x as i32 && 0 <= new_y && new_y < max_y as i32 {
                                if chr_line[new_x as usize][new_y as usize] != target[idx as usize] {
                                    is_success = false;
                                    break;
                                }
                            } else {
                                is_success = false;
                                break;
                            }
                        }
                    }

                    if is_success {
                        answer += 1;
                        break;
                    }
                }
            }
        }
    }

    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day4_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase("testcase/day4/part1/sample_in.txt", "testcase/day4/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase("testcase/day4/part1/real_in.txt", "testcase/day4/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase("testcase/day4/part2/sample_in.txt", "testcase/day4/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase("testcase/day4/part2/real_in.txt", "testcase/day4/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }
}
