use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> i32 {
    let mut coords = (0i16, 0i32, 0i16);  // (horizontal pos, depth, aim)
    for command in input {
        let mut parts = command.splitn(2, " ");
        match parts.next().unwrap() {
            // TODO: look for proper handling of these unwrap parses
            "forward" => {
                let val = parts.next().unwrap().parse::<i16>().unwrap();
                coords.0 += val;
                coords.1 += val as i32 * coords.2 as i32;
            },
            "down" => coords.2 += parts.next().unwrap().parse::<i16>().unwrap(),
            "up" => coords.2 -= parts.next().unwrap().parse::<i16>().unwrap(),
            _ => println!("Invalid command detected")
        }
    }

    return coords.0 as i32 * coords.1 as i32;
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 900);
}
