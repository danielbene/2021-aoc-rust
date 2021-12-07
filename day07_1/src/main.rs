use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> isize {
    let mut crab_subs = Vec::new();
    for split in input.get(0).unwrap().split(",") {
        crab_subs.push(split.parse().unwrap());
    }

    let med = aoc_util::median(&mut crab_subs);
    let mut fuel_cnt = 0;
    for pos in crab_subs.iter() {
        if *pos < med {
            fuel_cnt += med - pos;
        } else {
            fuel_cnt += pos - med;
        }
    }

    fuel_cnt
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 37);
}
