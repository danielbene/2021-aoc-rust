use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> isize {
    let mut counter = 0;
    for line in input {
        let disp_output = line.split(" | ").nth(1).unwrap();
        for value in disp_output.split(" ") {
            if [2, 3, 4, 7].contains(&value.len()) {
                counter += 1;
            }
        }
    }

    counter
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 26);
}
