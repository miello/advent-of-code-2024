const MAX_X: i64 = 101;
const MAX_Y: i64 = 103;

#[derive(Clone, Copy, Debug)]
struct Robot {
    pub x: i64,
    pub y: i64,
    speed_x: i64,
    speed_y: i64,
}

impl Robot {
    pub fn new(x: i64, y: i64, speed_x: i64, speed_y: i64) -> Self {
        Self {
            x,
            y,
            speed_x,
            speed_y,
        }
    }

    pub fn get_quadrant(&self) -> Option<usize> {
        match (
            self.x > MAX_X / 2,
            self.y > MAX_Y / 2,
            self.x == MAX_X / 2,
            self.y == MAX_Y / 2,
        ) {
            (_, _, true, _) => None,
            (_, _, _, true) => None,
            (false, false, _, _) => Some(0),
            (true, false, _, _) => Some(1),
            (false, true, _, _) => Some(2),
            (true, true, _, _) => Some(3),
        }
    }

    pub fn move_one(&mut self) {
        self.move_step(1);
    }

    pub fn move_step(&mut self, time: usize) {
        for _ in 0..time {
            self.x += self.speed_x;
            self.y += self.speed_y;

            while self.y < 0 {
                self.y += MAX_Y;
            }
            self.y %= MAX_Y;

            while self.x < 0 {
                self.x += MAX_X;
            }
            self.x %= MAX_X;
        }
    }
}

fn read_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (position, velocity) = line.split_once(" ").unwrap();

            let (x, y) = position[2..].split_once(",").unwrap();
            let (speed_x, speed_y) = velocity[2..].split_once(",").unwrap();

            Robot::new(
                x.parse::<i64>().unwrap(),
                y.parse::<i64>().unwrap(),
                speed_x.parse::<i64>().unwrap(),
                speed_y.parse::<i64>().unwrap(),
            )
        })
        .collect::<Vec<_>>()
}

fn part_one(input: String) -> String {
    let mut robots = read_input(&input);
    let mut quadrant_count: [i64; 4] = [0, 0, 0, 0];

    robots.iter_mut().for_each(|robot| {
        robot.move_step(100);
        if let Some(quadrant) = robot.get_quadrant() {
            quadrant_count[quadrant] += 1;
        }
    });

    let answer = quadrant_count[0] * quadrant_count[1] * quadrant_count[2] * quadrant_count[3];

    format!("{}", answer)
}

fn part_two(input: String) -> String {
    let mut robots = read_input(&input);
    let target = "#".repeat(31);

    let mut step = 1;
    loop {
        let mut row: Vec<i32> = Vec::new();
        row.resize(MAX_X as usize, 0);

        let mut whole_map: Vec<Vec<i32>> = Vec::new();

        whole_map.resize(MAX_Y as usize, row);
        robots.iter_mut().for_each(|robot| {
            robot.move_one();
            whole_map[robot.y as usize][robot.x as usize] += 1;
        });

        let map_string = whole_map
            .iter()
            .map(|f| {
                f.iter()
                    .map(|ch| match ch {
                        0 => String::from("."),
                        _ => String::from("#"),
                    })
                    .collect::<Vec<String>>()
                    .join("")
            })
            .collect::<Vec<_>>()
            .join("\n");

        if map_string.contains(&target) {
            return format!("{}", step);
        }
        step += 1;
    }
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day14_test {
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
            "testcase/day14/part1/sample_in.txt",
            "testcase/day14/part1/sample_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase(
            "testcase/day14/part1/real_in.txt",
            "testcase/day14/part1/real_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day14/part2/sample_in.txt",
            "testcase/day14/part2/sample_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase(
            "testcase/day14/part2/real_in.txt",
            "testcase/day14/part2/real_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }
}
