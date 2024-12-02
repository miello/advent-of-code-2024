import sys
import os

solution_template = """
fn part_one(input: String) -> String {{
    return input
}}

fn part_two(input: String) -> String {{
    return input
}}

pub fn main() -> (fn(String) -> String, fn(String) -> String) {{
    (part_one, part_two)
}}

#[cfg(test)]
mod {name}_test {{
    use super::*;
    use std::fs;

    fn read_testcase(path_in: &str, path_out: &str) -> (String, String) {{
        (fs::read_to_string(path_in).expect("Unable to read input"), fs::read_to_string(path_out).expect("Unable to read output"))
    }}

    #[test]
    fn part_one_sample_test() {{
        let (input, output) = read_testcase("testcase/{name}/part1/sample_in.txt", "testcase/{name}/part1/sample_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }}

    #[test]
    fn part_one_real_test() {{
        let (input, output) = read_testcase("testcase/{name}/part1/real_in.txt", "testcase/{name}/part1/real_out.txt");
        assert_eq!(output.trim(), part_one(input));
    }}

    #[test]
    fn part_two_sample_test() {{
        let (input, output) = read_testcase("testcase/{name}/part2/sample_in.txt", "testcase/{name}/part2/sample_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }}

    #[test]
    fn part_two_real_test() {{
        let (input, output) = read_testcase("testcase/{name}/part2/real_in.txt", "testcase/{name}/part2/real_out.txt");
        assert_eq!(output.trim(), part_two(input));
    }}
}}
"""

def main():
    if len(sys.argv) != 2:
        print("Usage: python3 gen.py <filename>")

    # Generate new solution file
    filename = sys.argv[1].strip()

    solution_file_content = solution_template.format(name=filename)

    fp = open(f"src/solution/{filename}.rs", 'w')
    fp.write(solution_file_content)

    # Create testcase file
    testcase_name = [
        "real_in.txt",
        "real_out.txt",
        "sample_in.txt",
        "sample_out.txt"
    ]

    folder_name = [
        "part1",
        "part2"
    ]

    for folder in folder_name:
        if not os.path.exists(f"testcase/{filename}/{folder}"):
            os.makedirs(f"testcase/{filename}/{folder}")

        for testcase in testcase_name:
            open(f"testcase/{filename}/{folder}/{testcase}", 'w').close()

    # Append new solution to mod file
    mod_file_content = open('src/solution/mod.rs', 'a')
    mod_file_content.write("""
pub mod {name};""".format(name=filename))


if __name__ == "__main__":
    main()