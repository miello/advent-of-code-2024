use std::collections::HashSet;

#[derive(Debug, PartialEq, Clone, Copy)]
enum MapState {
    UNVISITED,
    VISITED,
    OBSTACLE,
    STARTER,
}

fn read_input(input: &str) -> Vec<Vec<MapState>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|f| !f.eq(&""))
                .map(|f| match f {
                    "#" => MapState::OBSTACLE,
                    "^" => MapState::STARTER,
                    _ => MapState::UNVISITED,
                })
                .collect()
        })
        .collect()
}

const DIFF: [(i32, i32); 4] = [
    (-1, 0),
    (0, 1),
    (1, 0),
    (0, -1),
];

fn part_one(input: String) -> String {
    let mut pos_x: i32 = 0;
    let mut pos_y: i32 = 0;
    let mut input_line: Vec<Vec<MapState>> = read_input(&input);

    for i in 0..input_line.len() {
        for j in 0..input_line[0].len() {
            if input_line[i][j] == MapState::STARTER {
                pos_x = i as i32;
                pos_y = j as i32;
                input_line[i][j] = MapState::UNVISITED;
            }
        }
    }

    let max_x: i32 = input_line.len().try_into().unwrap();
    let max_y: i32 = input_line[0].len().try_into().unwrap();

    let mut cur_dir: usize = 0;
    let mut count_walk = 0;

    loop {
        if pos_x < 0 || pos_x >= max_x || pos_y < 0 || pos_y >= max_y {
            break;
        }

        let cur_field = input_line[pos_x as usize][pos_y as usize];

        if cur_field == MapState::OBSTACLE {
            pos_x -= DIFF[cur_dir].0;
            pos_y -= DIFF[cur_dir].1;

            cur_dir += 1;
            if cur_dir == 4 {
                cur_dir = 0;
            }
            continue;
        } else if cur_field == MapState::UNVISITED {
            count_walk += 1;
        }

        input_line[pos_x as usize][pos_y as usize] = MapState::VISITED;
        pos_x += DIFF[cur_dir].0;
        pos_y += DIFF[cur_dir].1;
    }
    format!("{}", count_walk)
}

fn simulate_cycle(board: Vec<Vec<MapState>>, start_x: i32, start_y: i32) -> bool {
    let max_x: i32 = board.len().try_into().unwrap();
    let max_y: i32 = board[0].len().try_into().unwrap();

    let mut pos_x = start_x;
    let mut pos_y = start_y;

    let mut cur_dir = 0;

    // x, y, direction
    let mut pass_edge: HashSet<(i32, i32, usize)> = HashSet::new();

    loop {
        if pos_x < 0 || pos_x >= max_x || pos_y < 0 || pos_y >= max_y {
            break;
        }

        let cur_field = board[pos_x as usize][pos_y as usize];

        if cur_field == MapState::OBSTACLE {
            pos_x -= DIFF[cur_dir].0;
            pos_y -= DIFF[cur_dir].1;
            
            cur_dir += 1;
            if cur_dir == 4 {
                cur_dir = 0;
            }

            if pass_edge.contains(&(pos_x, pos_y, cur_dir)) {
                return true;
            }
            pass_edge.insert((pos_x, pos_y, cur_dir));
            continue;
        }

        pos_x += DIFF[cur_dir].0;
        pos_y += DIFF[cur_dir].1;
    }

    false   
}

fn part_two(input: String) -> String {
    let mut start_x: i32 = 0;
    let mut start_y: i32 = 0;
    let mut input_line: Vec<Vec<MapState>> = read_input(&input);

    for i in 0..input_line.len() {
        for j in 0..input_line[0].len() {
            if input_line[i][j] == MapState::STARTER {
                start_x = i as i32;
                start_y = j as i32;
                input_line[i][j] = MapState::UNVISITED;
            }
        }
    }

    let mut simulated_board = input_line.clone();

    let max_x: i32 = input_line.len().try_into().unwrap();
    let max_y: i32 = input_line[0].len().try_into().unwrap();

    let mut pos_x = start_x;
    let mut pos_y = start_y;

    let mut cur_dir: usize = 0;
    let mut count_block = 0;

    loop {
        if pos_x < 0 || pos_x >= max_x || pos_y < 0 || pos_y >= max_y {
            break;
        }

        let cur_field = input_line[pos_x as usize][pos_y as usize];

        if cur_field == MapState::OBSTACLE {
            pos_x -= DIFF[cur_dir].0;
            pos_y -= DIFF[cur_dir].1;

            cur_dir += 1;
            if cur_dir == 4 {
                cur_dir = 0;
            }       
            continue;
        }

        // If visited it means that previous path should prevent new line from happened 
        if (start_x != pos_x || start_y != pos_y) && cur_field != MapState::VISITED {
            simulated_board[pos_x as usize][pos_y as usize] = MapState::OBSTACLE;
            if simulate_cycle(simulated_board.clone(), start_x, start_y) {
                count_block += 1;
            }
            simulated_board[pos_x as usize][pos_y as usize] = MapState::UNVISITED;
            
        }

        input_line[pos_x as usize][pos_y as usize] = MapState::VISITED;
        pos_x += DIFF[cur_dir].0;
        pos_y += DIFF[cur_dir].1;
    }

    format!("{}", count_block)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day6_test {
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
            "testcase/day6/part1/sample_in.txt",
            "testcase/day6/part1/sample_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase(
            "testcase/day6/part1/real_in.txt",
            "testcase/day6/part1/real_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day6/part2/sample_in.txt",
            "testcase/day6/part2/sample_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase(
            "testcase/day6/part2/real_in.txt",
            "testcase/day6/part2/real_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }
}
