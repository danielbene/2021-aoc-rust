use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> i16 {
    let mut counter: i16 = -1;
    let mut prev: i16 = 0;
    let item_cnt = input.len();

    for measurement in 0..item_cnt-2 {
        let mut current: i16 = 0;
        for i in 0..3 {
            current += input.get(measurement + i).unwrap().parse::<i16>().unwrap();
        }

        if current > prev {
            counter += 1;
        }

        prev = current;
    }

    return counter;
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 5);
}
