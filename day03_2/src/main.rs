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

fn solve(input: &mut Vec<String>) -> u32{
    let oxy_gen = u16::from_str_radix(&search_rating(input, true), 2).unwrap();
    let co2_scrub = u16::from_str_radix(&search_rating(input, false), 2).unwrap();

    oxy_gen as u32 * co2_scrub as u32
}

fn search_rating(input: &mut Vec<String>, is_oxy: bool) -> String{
    let mut vec = input.to_owned();
    for i in 0..vec.get(0).unwrap().len() {
        let mut val = 0;
        for bin in vec.iter() {
            val += bin.chars().nth(i as usize).unwrap().to_digit(10).unwrap() as u16;
        }

        if val as f32 >= (vec.len() as f32 / 2.0) {
            remove_if_eq(&mut vec, i as u8, !is_oxy as u8);
        } else {
            remove_if_eq(&mut vec, i as u8, is_oxy as u8);
        }

        if vec.len() == 1 {
            break
        }
    }

    vec.get(0).unwrap().to_string()
}

fn remove_if_eq(vec: &mut Vec<String>, index: u8, val: u8) {
    let mut rem_index: Vec<u16> = Vec::new();
    for (i, binary) in vec.iter().enumerate() {
        // omg
        if (binary.chars().nth(index as usize).unwrap().to_digit(10).unwrap() as u8) == val {
            rem_index.push(i as u16);
        }
    }

    rem_index.sort();
    rem_index.reverse();
    for i in rem_index {
        vec.remove(i as usize);
    }
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
    assert!(solve(&mut vec) == 230);
}
