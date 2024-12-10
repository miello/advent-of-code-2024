use std::collections::HashSet;

fn read_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|f| !f.eq(&""))
                .map(|f| f.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect()
}

const WALK_PATH: [(i32, i32); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn trail_walking(board: &Vec<Vec<u8>>, already_count: &mut HashSet<(usize, usize)>, x: usize, y: usize, max_x: i32, max_y: i32) -> u64 {
    if board[x][y] == 9 && !already_count.contains(&(x, y)) {
        already_count.insert((x, y));
        return 1;
    }

    let mut answer = 0;

    for i in 0..4 {
        let new_x = x as i32 + WALK_PATH[i].0;
        let new_y = y as i32 + WALK_PATH[i].1;

        if 0 <= new_x && new_x < max_x && 0 <= new_y && new_y < max_y {
            if board[x][y] + 1 == board[new_x as usize][new_y as usize] {
                answer += trail_walking(board, already_count, new_x as usize, new_y as usize, max_x, max_y);
            }
        }
    }

    answer
}

fn part_one(input: String) -> String {
    let board = read_input(&input);

    let max_x = board.len();
    let max_y = board[0].len();

    let mut answer = 0;
    for i in 0..max_x {
        for j in 0..max_y {
            if board[i][j] == 0 {
                let mut already_count = HashSet::new();
                answer += trail_walking(&board, &mut already_count, i, j, max_x as i32, max_y as i32);
            }
        }
    }
 
    format!("{}", answer)
}

fn trail_walking_path(board: &Vec<Vec<u8>>, x: usize, y: usize, max_x: i32, max_y: i32) -> u64 {
    if board[x][y] == 9 {
        return 1;
    }

    let mut answer = 0;

    for i in 0..4 {
        let new_x = x as i32 + WALK_PATH[i].0;
        let new_y = y as i32 + WALK_PATH[i].1;

        if 0 <= new_x && new_x < max_x && 0 <= new_y && new_y < max_y {
            if board[x][y] + 1 == board[new_x as usize][new_y as usize] {
                answer += trail_walking_path(board, new_x as usize, new_y as usize, max_x, max_y);
            }
        }
    }

    answer
}

fn part_two(input: String) -> String {
    let board = read_input(&input);

    let max_x = board.len();
    let max_y = board[0].len();

    let mut answer = 0;
    for i in 0..max_x {
        for j in 0..max_y {
            if board[i][j] == 0 {
                answer += trail_walking_path(&board, i, j, max_x as i32, max_y as i32);
            }
        }
    }
 
    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day10_test {
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }

    #[test]
    fn part_one_sample_test() {
        let (input, output) = read_testcase("testcase/day10/part1/sample_in.txt", "testcase/day10/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase("testcase/day10/part1/real_in.txt", "testcase/day10/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase("testcase/day10/part2/sample_in.txt", "testcase/day10/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase("testcase/day10/part2/real_in.txt", "testcase/day10/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }
}
