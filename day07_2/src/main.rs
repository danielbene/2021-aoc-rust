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

    // rounding causes inconsistent result so using comparison to make sure
    // (test was ok with rounding, but live input failed and vice versa)
    let mean = aoc_util::mean(&mut crab_subs) as isize;
    let fuel_cnt = (calc_fuel(mean, &crab_subs), calc_fuel(mean + 1, &crab_subs));

    if fuel_cnt.0 < fuel_cnt.1 {
        fuel_cnt.0
    } else {
        fuel_cnt.1
    }
}

fn calc_fuel(test_pos: isize, vec: &Vec<isize>) -> isize{
    let mut fuel = 0;
    for pos in vec.iter() {
        if *pos < test_pos {
            fuel += triangle_num(test_pos - pos);
        } else {
            fuel += triangle_num(pos - test_pos);
        }
    }

    fuel
}

fn triangle_num(base: isize) -> isize {
    // #gomaths - src: https://math.stackexchange.com/questions/593318/factorial-but-with-addition/593323
    ((base * base) + base) / 2
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 168);
}
