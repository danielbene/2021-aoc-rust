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

// ##### MATH UTILS #####

pub fn median(numbers: &mut Vec<isize>) -> isize {
    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        mean(&vec![numbers[mid - 1], numbers[mid]]).round() as isize
    } else {
        numbers[mid]
    }
}

pub fn mean(numbers: &Vec<isize>) -> f64 {
    let sum = numbers.iter().sum::<isize>();
    sum as f64 / numbers.len() as f64
}

// ##### STRING UTILS #####

pub fn remove_whitespace(s: &str) -> String {
    s.chars().filter(|c| !c.is_whitespace()).collect()
}

pub fn contains_all_chars(target: &Vec<char>, sample: Vec<char>) -> bool {
    sample.iter().all(|item| target.contains(item))
}

pub fn count_matching_chars(target: &Vec<char>, sample: Vec<char>) -> u8 {
    let mut cnt: u8 = 0;
    for ch in sample.iter() {
        if target.contains(ch) {
            cnt += 1;
        }
    }

    cnt
}

pub fn copy_split_to_vec(string: &str, delimeter: &str) -> Vec<String> {
    let mut vec: Vec<String> = Vec::new();
    for s in string.split(delimeter) {
        vec.push(s.to_string());
    }

    vec
}

/*
NOTES:
- summing vector elements with conversion:
fishes.iter().map(|i| (*i) as u64).sum::<u64>()
- convert string specified char to num
bin.chars().nth(i as usize).unwrap().to_digit(10).unwrap() as u16;  // to_digit(RADIX)'s parameter is not totally clear
*/
