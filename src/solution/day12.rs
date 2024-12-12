use std::collections::HashSet;

fn read_input(input: &str) -> Vec<Vec<&str>> {
    input
        .lines()
        .map(|line| line.split("").filter(|f| !f.eq(&"")).collect())
        .collect()
}

const DIFF: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
const CHECK: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn flood_fill(
    input: &Vec<Vec<&str>>,
    pos: (i32, i32),
    passed_area: &mut Vec<Vec<bool>>,
) -> (i32, i32) {
    let max_x = input.len() as i32;
    let max_y = input[0].len() as i32;

    let mut area = 1;
    let mut perimeter = 0;

    let (x, y) = pos;
    let cur_text = input[x as usize][y as usize];

    passed_area[x as usize][y as usize] = true;

    for i in 0..4_usize {
        let new_x = x + DIFF[i].0;
        let new_y = y + DIFF[i].1;

        if 0 <= new_x
            && new_x < max_x
            && 0 <= new_y
            && new_y < max_y
            && cur_text.eq(input[new_x as usize][new_y as usize])
        {
            if !passed_area[new_x as usize][new_y as usize] {
                let (sub_area, sub_perimeter) = flood_fill(input, (new_x, new_y), passed_area);
                area += sub_area;
                perimeter += sub_perimeter;
            }
        } else {
            perimeter += 1;
        }
    }

    return (area, perimeter);
}

fn part_one(input: String) -> String {
    let board = read_input(&input);
    let max_x = board.len();
    let max_y = board[0].len();

    let mut answer = 0;

    let mut passed_area: Vec<Vec<bool>> = Vec::new();
    let mut passed_area_row = Vec::new();

    passed_area_row.resize(max_y, false);
    passed_area.resize(max_y, passed_area_row);

    for i in 0..max_x {
        for j in 0..max_y {
            if !passed_area[i][j] {
                let (area, perimeter) = flood_fill(&board, (i as i32, j as i32), &mut passed_area);
                answer += area * perimeter;
            }
        }
    }

    format!("{}", answer)
}

fn edge_count(
    input: &Vec<Vec<&str>>,
    pos: (i32, i32, usize),
    expected: (i32, i32, usize),
    is_start: bool,
    count: i32,
    duplicated: &mut HashSet<(i32, i32, usize)>,
) -> i32 {
    if pos == expected && !is_start {
        return count;
    }

    duplicated.insert(pos);

    let max_x = input.len() as i32;
    let max_y = input[0].len() as i32;

    let (x, y, direction) = pos;
    let cur_text: &str = input[x as usize][y as usize];

    let new_x = x + DIFF[direction].0;
    let new_y = y + DIFF[direction].1;

    let check_x = x + CHECK[direction].0;
    let check_y = y + CHECK[direction].1;

    if 0 <= check_x
        && check_x < max_x
        && 0 <= check_y
        && check_y < max_y
        && cur_text.eq(input[check_x as usize][check_y as usize])
    {
        let new_direction = (direction + 3) % 4;
        return edge_count(
            input,
            (check_x, check_y, new_direction),
            expected,
            false,
            count + 1,
            duplicated,
        );
    }

    if 0 <= new_x
        && new_x < max_x
        && 0 <= new_y
        && new_y < max_y
        && cur_text.eq(input[new_x as usize][new_y as usize])
    {
        return edge_count(
            input,
            (new_x, new_y, direction),
            expected,
            false,
            count,
            duplicated,
        );
    }

    let new_pos = (pos.0, pos.1, (pos.2 + 1) % 4);
    return edge_count(input, new_pos, expected, false, count + 1, duplicated);
}

fn flood_fill_part2(
    input: &Vec<Vec<&str>>,
    pos: (i32, i32),
    passed_area: &mut Vec<Vec<bool>>,
    passed_edge: &mut HashSet<(i32, i32, usize)>,
) -> (i32, i32) {
    let max_x = input.len() as i32;
    let max_y = input[0].len() as i32;

    let mut area = 1;
    let mut edge = 0;

    let (x, y) = pos;
    let cur_text: &str = input[x as usize][y as usize];

    passed_area[x as usize][y as usize] = true;

    let check_x = x + CHECK[0].0;
    let check_y = y + CHECK[0].1;

    let need_traverse_edge = (check_x < 0
        || check_x >= max_x
        || check_y < 0
        || check_y >= max_y
        || !cur_text.eq(input[check_x as usize][check_y as usize]))
        && !passed_edge.contains(&(x, y, 0));

    if need_traverse_edge {
        edge = edge_count(input, (x, y, 0), (x, y, 0), true, 0, passed_edge);
    }

    for i in 0..4_usize {
        let new_x = x + DIFF[i].0;
        let new_y = y + DIFF[i].1;

        if 0 <= new_x
            && new_x < max_x
            && 0 <= new_y
            && new_y < max_y
            && cur_text.eq(input[new_x as usize][new_y as usize])
            && !passed_area[new_x as usize][new_y as usize]
        {
            let (sub_area, sub_edge) =
                flood_fill_part2(input, (new_x, new_y), passed_area, passed_edge);

            area += sub_area;
            edge += sub_edge;
        }
    }

    return (area, edge);
}

fn part_two(input: String) -> String {
    let board = read_input(&input);
    let max_x = board.len();
    let max_y = board[0].len();

    let mut answer = 0;

    let mut passed_area: Vec<Vec<bool>> = Vec::new();
    let mut passed_area_row = Vec::new();

    passed_area_row.resize(max_y, false);
    passed_area.resize(max_y, passed_area_row);

    let mut passed_edge: HashSet<(i32, i32, usize)> = HashSet::new();

    for i in 0..max_x {
        for j in 0..max_y {
            if !passed_area[i][j] {
                let (area, edge) = flood_fill_part2(
                    &board,
                    (i as i32, j as i32),
                    &mut passed_area,
                    &mut passed_edge,
                );

                answer += area * edge;
            }
        }
    }

    format!("{}", answer)
}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {
    (part_one, part_two)
}

#[cfg(test)]
mod day12_test {
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
            "testcase/day12/part1/sample_in.txt",
            "testcase/day12/part1/sample_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_one_real_test() {
        let (input, output) = read_testcase(
            "testcase/day12/part1/real_in.txt",
            "testcase/day12/part1/real_out.txt",
        );
        assert_eq!(output.trim(), part_one(input));
    }

    #[test]
    fn part_two_sample_test() {
        let (input, output) = read_testcase(
            "testcase/day12/part2/sample_in.txt",
            "testcase/day12/part2/sample_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }

    #[test]
    fn part_two_real_test() {
        let (input, output) = read_testcase(
            "testcase/day12/part2/real_in.txt",
            "testcase/day12/part2/real_out.txt",
        );
        assert_eq!(output.trim(), part_two(input));
    }
}
