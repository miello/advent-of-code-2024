use core::panic;
use std::{thread, time};

#[derive(Clone, Copy, PartialEq, Debug)]
enum BoardState {
    EMPTY = 0,
    ROBOT = 1,
    ROCK = 2,
    WALL = 3,
    OPENBRACKET = 4,
    CLOSEBRACKET = 5,
}

#[derive(Clone, Copy, PartialEq, Debug)]
enum Direction {
    LEFT = 0,
    DOWN = 1,
    RIGHT = 2,
    UP = 3,
}

#[derive(Debug)]
struct Board {
    board: Vec<Vec<BoardState>>,
    size: (usize, usize),
    pos: (usize, usize),
}

impl Board {
    fn is_valid_pos(&self, x: i64, y: i64) -> bool {
        let (max_x, max_y) = self.size;
        return 0 <= x && x < max_x as i64 && 0 <= y && y < max_y as i64;
    }

    #[allow(dead_code)]
    pub fn print_board(&self) {
        let (x, y) = self.size;
        print!("\x1B[2J");
        for i in 0..x {
            for j in 0..y {
                print!(
                    "{}",
                    match self.board[i][j] {
                        BoardState::EMPTY => ".",
                        BoardState::ROCK => "O",
                        BoardState::ROBOT => "@",
                        BoardState::WALL => "#",
                        BoardState::OPENBRACKET => "[",
                        BoardState::CLOSEBRACKET => "]",
                    }
                )
            }
            println!();
        }
        println!("{:?}", self.pos);
        println!();
        let ten_millis = time::Duration::from_millis(10);

        thread::sleep(ten_millis);
    }

    pub fn calculate_score(&self) -> i64 {
        let (x, y) = self.size;
        let mut answer: i64 = 0;
        for i in 0..(x as usize) {
            for j in 0..(y as usize) {
                answer += match self.board[i][j] {
                    BoardState::ROCK | BoardState::OPENBRACKET => 100 * i as i64 + j as i64,
                    _ => 0,
                };
            }
        }

        answer
    }

    pub fn new_part_1(raw_board: Vec<&str>) -> Self {
        let mut pos = (0, 0);
        let mut board: Vec<Vec<BoardState>> = Vec::new();
        for (i, row) in raw_board.iter().enumerate() {
            let mut row_state: Vec<BoardState> = Vec::new();

            for (j, col) in row.split("").filter(|f| !f.eq(&"")).enumerate() {
                row_state.push(match col {
                    "#" => BoardState::WALL,
                    "." => BoardState::EMPTY,
                    "@" => {
                        pos = (i, j);
                        BoardState::ROBOT
                    }
                    "O" => BoardState::ROCK,
                    _ => panic!("Board state is not defined"),
                });
            }

            board.push(row_state);
        }

        let size = (board.len(), board[0].len());

        Self { board, size, pos }
    }

    pub fn move_robot_part_1(&mut self, direction: Direction) {
        let (dx, dy): (i64, i64) = match direction {
            Direction::DOWN => (1, 0),
            Direction::LEFT => (0, -1),
            Direction::UP => (-1, 0),
            Direction::RIGHT => (0, 1),
        };

        let (current_x, current_y) = self.pos;
        let (new_x, new_y) = (self.pos.0 as i64 + dx, self.pos.1 as i64 + dy);

        if self.is_valid_pos(new_x, new_y) {
            let new_x_usize = new_x as usize;
            let new_y_usize = new_y as usize;

            match self.board[new_x_usize][new_y_usize] {
                BoardState::EMPTY => {
                    let tmp = self.board[current_x][current_y];
                    self.board[current_x][current_y] = self.board[new_x_usize][new_y_usize];
                    self.board[new_x_usize][new_y_usize] = tmp;

                    self.pos = (new_x_usize, new_y_usize);
                }
                BoardState::ROCK => {
                    let mut candidate_x = new_x_usize;
                    let mut candidate_y = new_y_usize;

                    while self.is_valid_pos(candidate_x as i64, candidate_y as i64)
                        && self.board[candidate_x][candidate_y] == BoardState::ROCK
                    {
                        candidate_x = (candidate_x as i64 + dx) as usize;
                        candidate_y = (candidate_y as i64 + dy) as usize;
                    }

                    if self.is_valid_pos(candidate_x as i64, candidate_y as i64)
                        && self.board[candidate_x][candidate_y] == BoardState::EMPTY
                    {
                        loop {
                            let prev_pos_x = (candidate_x as i64 - dx) as usize;
                            let prev_pos_y = (candidate_y as i64 - dy) as usize;

                            let tmp = self.board[candidate_x][candidate_y];
                            self.board[candidate_x][candidate_y] =
                                self.board[prev_pos_x][prev_pos_y];
                            self.board[prev_pos_x][prev_pos_y] = tmp;

                            if self.board[candidate_x][candidate_y] == BoardState::ROBOT {
                                break;
                            }

                            candidate_x = prev_pos_x;
                            candidate_y = prev_pos_y;
                        }

                        self.pos = (candidate_x, candidate_y);
                    }
                }
                _ => {}
            }
        }
    }

    pub fn new_part_2(raw_board: Vec<&str>) -> Self {
        let mut pos = (0, 0);
        let mut board: Vec<Vec<BoardState>> = Vec::new();
        for (i, row) in raw_board.iter().enumerate() {
            let mut row_state: Vec<BoardState> = Vec::new();

            for (j, col) in row.split("").filter(|f| !f.eq(&"")).enumerate() {
                row_state.append(&mut match col {
                    "#" => vec![BoardState::WALL, BoardState::WALL],
                    "." => vec![BoardState::EMPTY, BoardState::EMPTY],
                    "@" => {
                        pos = (i, j);
                        vec![BoardState::ROBOT, BoardState::EMPTY]
                    }
                    "O" => vec![BoardState::OPENBRACKET, BoardState::CLOSEBRACKET],
                    _ => panic!("Board state is not defined"),
                });
            }

            board.push(row_state);
        }

        let size = (board.len(), board[0].len());

        Self { board, size, pos }
    }

    pub fn move_robot_part_2(direction: Direction) {

    }
}

fn read_input(input: &str) -> (Vec<&str>, Vec<Direction>) {
    let mut raw_board: Vec<&str> = Vec::new();
    let mut robot_direction: Vec<Direction> = Vec::new();
    let mut is_board_input = true;

    input.lines().for_each(|f| {
        if f.trim() == "" {
            is_board_input = false;
            return;
        }

        if is_board_input {
            raw_board.push(f.trim());
        } else {
            robot_direction.append(
                &mut f
                    .split("")
                    .filter(|ch| !ch.eq(&""))
                    .map(|ch| match ch {
                        "<" => Direction::LEFT,
                        ">" => Direction::RIGHT,
                        "^" => Direction::UP,
                        "v" => Direction::DOWN,
                        _ => panic!("Invalid input {}", ch),
                    })
                    .collect::<Vec<_>>(),
            );
        }
    });

    (raw_board, robot_direction)
}
fn part_one(input: String) -> String {
    let (raw_board, input_direction) = read_input(&input);
    let mut board = Board::new_part_1(raw_board);

    for direction in input_direction {
        board.move_robot_part_1(direction);
        board.print_board();
    }

    format!("{}", board.calculate_score())
}

fn part_two(input: String) -> String {
    let (raw_board, input_direction) = read_input(&input);
    let mut board = Board::new_part_2(raw_board);

    for direction in input_direction {
        // board.move_robot(direction);
        board.print_board();
    }

    format!("{}", board.calculate_score())
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day15_test {
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
            "testcase/day15/part1/sample_in.txt",
            "testcase/day15/part1/sample_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase(
            "testcase/day15/part1/real_in.txt",
            "testcase/day15/part1/real_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day15/part2/sample_in.txt",
            "testcase/day15/part2/sample_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase(
            "testcase/day15/part2/real_in.txt",
            "testcase/day15/part2/real_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }
}
