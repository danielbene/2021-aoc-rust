use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

// will not update part1 because it's nice to see the runtime difference
// between the "manual" and this counting solution
fn solve(input: &mut Vec<String>) -> isize {
    let mut fishes = vec!(0; 7);
    for spit in input.get(0).unwrap().split(",") {
        fishes[spit.parse::<usize>().unwrap()] += 1;
    }

    let mut newborns = (0, 0);
    for _ in 0..256 {
        let newborn_cnt = fishes[0];
        fishes.rotate_left(1);
        fishes[6] += newborns.0;
        newborns.0 = newborns.1;
        newborns.1 = newborn_cnt;
    }

    let fish_sum = fishes.iter().map(|i| (*i) as usize).sum::<usize>() + newborns.0 + newborns.1;
    fish_sum as isize
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 26984457539);
}
