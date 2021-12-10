use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> isize {
    let mut incorrect_chars: Vec<char> = Vec::new();
    let mut cnt = 0;
    for line in input.iter(){
        get_incorrect_closers(&remove_pairs(line), &mut incorrect_chars);
    }

    for ch in incorrect_chars.iter() {
        match ch {
            ')' => cnt += 3,
            ']' => cnt += 57,
            '}' => cnt += 1197,
            '>' => cnt += 25137,
            _ => ()
        }
    }

    cnt
}

fn get_incorrect_closers(cleaned: &String, incorrect_chars: &mut Vec<char>) {
    let closers = [']', ')', '}', '>'];
    let mut idx: Vec<u8> = Vec::new();
    for c in closers {
        if cleaned.contains(c) {
            let i = cleaned.chars().position(|ch| ch == c).unwrap() as u8;
            idx.push(i);
        }
    }

    if idx.len() != 0 {
        idx.sort();
        incorrect_chars.push(cleaned.chars().nth(idx[0] as usize).unwrap());
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
    assert!(solve(&mut vec) == 26397);
}
