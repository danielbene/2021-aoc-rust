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

fn load_from_file(file_path: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Something went wrong reading the file");
    let reader = BufReader::new(file);

    // yep, it's a frickin return, get used to it
    reader.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

#[test]
fn tests() {
    let test_input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let mut vec: Vec<String> = test_input.lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect();
    assert!(solve(&mut vec) == 5);
}
