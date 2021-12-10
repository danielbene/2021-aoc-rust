use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> isize {
    let mut incomplete_lines: Vec<String> = Vec::new();
    let mut cnt_vec: Vec<u32> = Vec::new();
    for line in input.iter(){
        get_incorrect_closers(&remove_pairs(line), &mut incomplete_lines);
    }

    for il in incomplete_lines {
        let ml = mirror_to_closers(&il);

        // TODO: calculate ml value, sort vec, return middle score


    }

    0
}

fn mirror_to_closers(line: &str) -> String {
    let mut mirrored = "".to_string();
    for ch in line.chars().rev() {
        match ch {
            '(' => mirrored.push(')'),
            '[' => mirrored.push(']'),
            '{' => mirrored.push('}'),
            '<' => mirrored.push('>'),
            _ => ()
        }
    }

    mirrored
}

fn get_incorrect_closers(cleaned: &String, incomplete_lines: &mut Vec<String>) {
    let closers = [']', ')', '}', '>'];
    let mut idx: Vec<u8> = Vec::new();
    for c in closers {
        if cleaned.contains(c) {
            let i = cleaned.chars().position(|ch| ch == c).unwrap() as u8;
            idx.push(i);
        }
    }

    if idx.len() == 0 {
        incomplete_lines.push(cleaned.to_string());
    }
}

fn remove_pairs(line: &str) -> String{
    let pairs = ["[]", "()", "{}", "<>"];
    let mut removed = true;
    let mut tmp: String = line.to_string();
    while removed {
        removed = false;
        for pair in pairs {
            let len = tmp.len();
            tmp = tmp.replace(pair, "");
            if tmp.len() != len {
                removed = true;
            }
        }
    }

    tmp
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 288957);
}
