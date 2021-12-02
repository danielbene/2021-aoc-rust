use std::{
    fs,
    time::Instant,
    io::BufReader,
    io::BufRead
};

fn main() {
    let mut lines_vec = load_from_file("input.txt");

    let start = Instant::now();
    let solution = solve(&mut lines_vec);
    let runtime = Instant::now() - start;

    let output = format!("Solution: {}\nRuntime: {}ms ({} microsec)\n",
        solution, runtime.as_millis(), runtime.as_micros());
    fs::write("solution.txt", output).expect("Unable to write file");
}

fn solve(input: &mut Vec<String>) -> i32 {
    let mut coords = (0i16, 0i16);  // (horizontal pos, depth)
    for command in input {
        let mut parts = command.splitn(2, " ");
        match parts.next().unwrap() {
            // TODO: look for proper handling of these unwrap parses
            "forward" => coords.0 += parts.next().unwrap().parse::<i16>().unwrap(),
            "down" => coords.1 += parts.next().unwrap().parse::<i16>().unwrap(),
            "up" => coords.1 -= parts.next().unwrap().parse::<i16>().unwrap(),
            _ => println!("Invalid command detected")
        }
    }

    return coords.0 as i32 * coords.1 as i32;
}

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Something went wrong reading the file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

#[test]
fn tests() {
    let mut vec: Vec<String> = load_from_file("test.txt");
    assert!(solve(&mut vec) == 150);
}
