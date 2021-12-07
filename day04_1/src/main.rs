use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

// welp, this is NOT nice
fn solve(input: &mut Vec<String>) -> isize {
    let mut table_stats: Vec<(u8, i8, u16)> = Vec::new();
    let mut nums: Vec<u8> = Vec::new();
    for n in input.get(0).unwrap().split(",") {
        nums.push(n.parse().unwrap());
    }

    for (t_index, t_line_index) in (2..input.len()).step_by(6).enumerate() {
        let max_weight = nums.len();
        let mut lowest_weight: i8 = max_weight as i8;
        let mut column_weights = vec!(0i8; 5);

        for i in t_line_index..t_line_index + 5 {
            // test lines
            let mut highest_index: i8 = 0;
            let mut col_i = 0;  // cant use enumerate bc double spaces
            for line_num in input.get(i).unwrap().split(" ") {
                if line_num == "" {
                    continue
                }

                let n: u8 = aoc_util::remove_whitespace(line_num).parse().unwrap();
                if nums.contains(&n) {
                    let h_idx: i8 = nums.iter().position(|&x| x == n).unwrap() as i8;  // index of element in vector
                    if h_idx > highest_index {
                        highest_index = h_idx;
                    }

                    if *column_weights.get(col_i).unwrap() != -1i8 && h_idx > *column_weights.get(col_i).unwrap() {
                        column_weights[col_i] = h_idx;
                    }
                } else {
                    highest_index = -1;
                    column_weights[col_i] = -1;
                    break
                }

                col_i += 1;
            }

            if highest_index != -1 && highest_index < lowest_weight {
                lowest_weight = highest_index;
            }
        }

        // compare table column weights
        for col_w in column_weights.iter() {
            if *col_w < lowest_weight {
                lowest_weight = *col_w;
            }
        }

        let mut table_sum: u16 = 0;
        for i in t_line_index..t_line_index + 5 {
            for line_num in input.get(i).unwrap().split(" ") {
                if line_num == "" {
                    continue
                }

                let n: u8 = aoc_util::remove_whitespace(line_num).parse().unwrap();
                if !nums[0..=lowest_weight as usize].contains(&n) {
                    table_sum += n as u16;
                }
            }
        }

        table_stats.push((t_index as u8, lowest_weight, table_sum));
    }

    table_stats.sort_by_key(|k| k.1);

    let winner = table_stats.get(0).unwrap();
    *nums.get(winner.1 as usize).unwrap() as isize * winner.2 as isize
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 4512);
}
