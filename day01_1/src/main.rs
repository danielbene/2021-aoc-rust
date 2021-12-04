use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> i16 {
    let mut counter: i16 = -1;
    let mut prev: i16 = 0;

    for line in input.iter() {
        let val: i16 = line.parse().unwrap();  // line.parse::<i16>().unwrap()
        if val > prev {
            counter += 1;
        }

        prev = val;
    }

    return counter;
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 7);
}
