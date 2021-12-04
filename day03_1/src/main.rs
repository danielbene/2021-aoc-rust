use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
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

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 198);
}
