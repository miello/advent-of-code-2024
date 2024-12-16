use std::{
    cmp::{min, Ordering},
    collections::{BinaryHeap, HashSet},
    i64,
};

#[derive(PartialEq, Eq, PartialOrd, Clone, Copy)]
enum BoardState {
    EMPTY,
    WALL,
    START,
    GOAL,
}

#[derive(PartialEq, PartialOrd, Eq)]
struct Node {
    pub cost: i64,
    pub direction: i8,
    pub pos: (usize, usize),
}

impl Node {
    pub fn new(cost: i64, pos: (usize, usize), direction: i8) -> Self {
        Self {
            cost,
            pos,
            direction,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

const DIFF: [(i64, i64); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

fn read_input(input: &str) -> Vec<Vec<BoardState>> {
    input
        .lines()
        .map(|line| {
            line.split("")
                .filter(|ch| !ch.eq(&""))
                .map(|ch| match ch {
                    "#" => BoardState::WALL,
                    "." => BoardState::EMPTY,
                    "S" => BoardState::START,
                    "E" => BoardState::GOAL,
                    _ => panic!("Invalid board state: {}", ch),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn generate_shortest_path(board: &Vec<Vec<BoardState>>) -> Vec<Vec<[i64; 4]>> {
    let mut board_row: Vec<[i64; 4]> = Vec::new();
    board_row.resize(board[0].len(), [i64::MAX, i64::MAX, i64::MAX, i64::MAX]);

    let mut board_cost: Vec<Vec<[i64; 4]>> = Vec::new();
    board_cost.resize(board.len(), board_row);

    let mut pq: BinaryHeap<Node> = BinaryHeap::new();

    let max_x = board.len() as i64;
    let max_y = board[0].len() as i64;

    for i in 0..max_x as usize {
        for j in 0..max_y as usize {
            if board[i][j] == BoardState::START {
                board_cost[i][j][1] = 0;
                pq.push(Node::new(0, (i, j), 1));
            }
        }
    }

    while !pq.is_empty() {
        let node = pq.pop().unwrap();

        let direction = node.direction;
        let (x, y) = node.pos;

        for diff in [-1, 0, 1] {
            let walk_cost = match diff {
                0 => 1,
                _ => 1000,
            };

            let new_direction = (diff + direction + 4) % 4;
            let new_x = match diff {
                0 => (x as i64) + DIFF[new_direction as usize].0,
                _ => x as i64,
            };
            let new_y = match diff {
                0 => (y as i64) + DIFF[new_direction as usize].1,
                _ => y as i64,
            };

            if 0 <= new_x && new_x < max_x && 0 <= new_y && new_y < max_y {
                let new_x_usize = new_x as usize;
                let new_y_usize = new_y as usize;

                if board[new_x_usize][new_y_usize] != BoardState::WALL
                    && board_cost[new_x_usize][new_y_usize][new_direction as usize]
                        > board_cost[x][y][direction as usize] + walk_cost
                {
                    board_cost[new_x_usize][new_y_usize][new_direction as usize] =
                        board_cost[x][y][direction as usize] + walk_cost;
                    pq.push(Node::new(
                        -board_cost[new_x_usize][new_y_usize][new_direction as usize],
                        (new_x_usize, new_y_usize),
                        new_direction,
                    ));
                }
            }
        }
    }

    return board_cost;
}

fn part_one(input: String) -> String {
    let board = read_input(&input);

    let max_x = board.len();
    let max_y = board[0].len();

    let result = generate_shortest_path(&board);
    let mut answer = i64::MAX;

    for i in 0..max_x {
        for j in 0..max_y {
            if board[i][j] == BoardState::GOAL {
                for data in result[i][j] {
                    answer = min(answer, data);
                }
                break;
            }
        }
    }

    format!("{}", answer)
}

fn backtrack_to_end(
    board_cost: &Vec<Vec<[i64; 4]>>,
    pos: (usize, usize, i8),
    pos_mem: &mut HashSet<(usize, usize)>,
) -> bool {
    let (x, y, direction) = pos;
    if board_cost[x][y][direction as usize] == 0 {
        return true;
    }

    let max_x = board_cost.len() as i64;
    let max_y = board_cost[0].len() as i64;
    let mut is_found_answer = false;

    for diff in [-1, 0, 1] {
        let walk_cost = match diff {
            0 => 1,
            _ => 1000,
        };

        let new_direction = (direction - diff + 4) % 4;
        let new_x = match diff {
            0 => (x as i64) - DIFF[new_direction as usize].0,
            _ => x as i64,
        };
        let new_y = match diff {
            0 => (y as i64) - DIFF[new_direction as usize].1,
            _ => y as i64,
        };

        if 0 <= new_x && new_x < max_x && 0 <= new_y && new_y < max_y {
            let new_x_usize = new_x as usize;
            let new_y_usize = new_y as usize;

            if board_cost[x][y][direction as usize] - walk_cost
                == board_cost[new_x_usize][new_y_usize][new_direction as usize]
            {
                if backtrack_to_end(board_cost, (new_x_usize, new_y_usize, new_direction), pos_mem) {
                    pos_mem.insert((new_x_usize, new_y_usize));
                    is_found_answer = true;
                }
            }
        }
    }

    if is_found_answer {
        return true;
    }

    return false;
}

fn part_two(input: String) -> String {
    let board = read_input(&input);

    let max_x = board.len();
    let max_y = board[0].len();

    let result = generate_shortest_path(&board);
    let mut pos_mem: HashSet<(usize, usize)> = HashSet::new();

    for i in 0..max_x {
        for j in 0..max_y {
            if board[i][j] == BoardState::GOAL {
                for direction in 0..4 {
                    backtrack_to_end(&result, (i, j, direction), &mut pos_mem);
                }
                break;
            }
        }
    }

    format!("{}", pos_mem.len())
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day16_test {
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
            "testcase/day16/part1/sample_in.txt",
            "testcase/day16/part1/sample_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase(
            "testcase/day16/part1/real_in.txt",
            "testcase/day16/part1/real_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day16/part2/sample_in.txt",
            "testcase/day16/part2/sample_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase(
            "testcase/day16/part2/real_in.txt",
            "testcase/day16/part2/real_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }
}
