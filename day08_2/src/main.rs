use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

// this is really woodcutter, probably suboptimal
fn solve(input: &mut Vec<String>) -> isize {
    let mut counter = 0;
    for line in input {
        let mut split = line.split(" | ");
        let mut num_segments: Vec<String> = vec!("".to_string(); 10);

        let patterns = aoc_util::copy_split_to_vec(split.next().unwrap(), " ");
        get_unique_segment_values(&patterns, &mut num_segments);
        test_fives(&patterns, &mut num_segments);
        test_sixes(&patterns, &mut num_segments);

        let disp_output = split.next().unwrap();
        for (i, value) in disp_output.split(" ").enumerate() {
            for (j, num) in num_segments.iter().enumerate() {
                if value.len() == num.len() && aoc_util::contains_all_chars(&value.chars().collect(), num.chars().collect())  {
                    counter += (j as u16 * 10u16.pow(3 - i as u32)) as isize;
                    break
                }
            }
        }
    }

    counter
}

fn get_unique_segment_values(patterns: &Vec<String>, vec: &mut Vec<String>){
    for signal in patterns {
        match signal.len() {
            2 => vec[1] = signal.to_string(),
            3 => vec[7] = signal.to_string(),
            4 => vec[4] = signal.to_string(),
            7 => vec[8] = signal.to_string(),
            _ => ()
        }
    }
}

fn test_fives(patterns: &Vec<String>, vec: &mut Vec<String>) {
    for signal in patterns {
        if signal.len() == 5 {
            let target = signal.chars().collect();
            if aoc_util::contains_all_chars(&target, vec[1].chars().collect()) {
                vec[3] = signal.to_string();
            } else if aoc_util::count_matching_chars(&target, vec[4].chars().collect()) == 3 {
                vec[5] = signal.to_string();
            } else {
                vec[2] = signal.to_string();
            }
        }
    }
}

fn test_sixes(patterns: &Vec<String>, vec: &mut Vec<String>) {
    for signal in patterns {
        if signal.len() == 6 {
            let target = signal.chars().collect();
            if aoc_util::count_matching_chars(&target, vec[4].chars().collect()) == 4 {
                vec[9] = signal.to_string();
            } else if aoc_util::count_matching_chars(&target, vec[7].chars().collect()) == 3 {
                vec[0] = signal.to_string();
            } else {
                vec[6] = signal.to_string();
            }
        }
    }
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 61229);
}
