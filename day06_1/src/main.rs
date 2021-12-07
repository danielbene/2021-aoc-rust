use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> isize {
    let mut fishes: Vec<u8> = Vec::new();
    for split in input.get(0).unwrap().split(",") {
        fishes.push(split.parse().unwrap());
    }

    for _ in 0..80 {
        let mut newborn = 0;
        for fish in &mut fishes {
            if *fish == 0 {
                *fish = 6;
                newborn += 1;
            } else {
                *fish -= 1;
            }
        }

        for _ in 0..newborn {
            fishes.push(8);
        }
    }

    fishes.len() as isize
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 5934);
}
