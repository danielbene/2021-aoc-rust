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
    let bin_len = input.get(0).unwrap().len();
    let commonity_value: i16 = (input.len() / 2) as i16;
    let mut counter_vec = vec![0i16; bin_len];  // index 0 most left (highest) bit
    for binary in input {
        for (i, c) in binary.chars().enumerate() {
            counter_vec[i] += c.to_digit(10).unwrap() as i16;  // index 0 is the most left bit
        }
    }

    let mut gamma_bin_str: String = "".to_owned();
    let mut epsilon_bin_str: String = "".to_owned();
    for val in &mut counter_vec {
        gamma_bin_str.push_str(&((*val > commonity_value) as u8).to_string());
        epsilon_bin_str.push_str(&(!(*val > commonity_value) as u8).to_string());
    }

    let gamma = u16::from_str_radix(&gamma_bin_str, 2).unwrap();
    let epsilon = u16::from_str_radix(&epsilon_bin_str, 2).unwrap();

    return gamma as i32 * epsilon as i32;
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
    assert!(solve(&mut vec) == 198);
}
