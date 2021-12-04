use std::{
    fs,
    time::Instant,
    io::BufReader,
    io::BufRead
};

pub fn init() -> (Vec<String>, Instant) {
    (load_from_file("input.txt"), Instant::now())
}

pub fn end(solution: isize, start: Instant) {
    let runtime = Instant::now() - start;
    let output = format!("Solution: {}\nRuntime: {}ms ({} microsec)\n",
        solution, runtime.as_millis(), runtime.as_micros());
    fs::write("solution.txt", output).expect("Unable to write file");
}

pub fn load_from_file(file_path: &str) -> Vec<String> {
    let file = fs::File::open(file_path).expect("Something went wrong reading the file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}
