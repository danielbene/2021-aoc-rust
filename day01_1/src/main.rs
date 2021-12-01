use std::{
    fs,
    time::Instant
};

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let start = Instant::now();
    let solution = solve(&contents);
    let runtime = Instant::now() - start;

    let output = format!("Solution: {}\nRuntime: {}ms ({} microsecs)",
        solution, runtime.as_millis(), runtime.as_micros());
    fs::write("solution.txt", output).expect("Unable to write file");
}

fn solve(input: &str) -> i16 {
    let mut counter: i16 = -1;
    let mut prev: i16 = 0;

    for line in input.lines() {
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
    let test_input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    assert!(solve(test_input) == 7);
}
