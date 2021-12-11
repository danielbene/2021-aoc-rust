use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> isize {
    let mut incomplete_lines: Vec<String> = Vec::new();
    let mut scores: Vec<usize> = Vec::new();
    for line in input.iter(){
        get_incomplete_closers(&remove_pairs(line), &mut incomplete_lines);
    }

    for il in incomplete_lines {
        scores.push(score_mirrored_value(&il));
    }

    scores.sort();
    scores[scores.len()/2] as isize
}

fn score_mirrored_value(line: &str) -> usize {
    let mut score: usize = 0;
    for ch in line.chars().rev() {
        score *= 5;
        match ch {
            '(' => score += 1,
            '[' => score += 2,
            '{' => score += 3,
            '<' => score += 4,
            _ => ()
        }
    }

    score
}

fn get_incomplete_closers(cleaned: &String, incomplete_lines: &mut Vec<String>) {
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
