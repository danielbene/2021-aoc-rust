use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> isize {
    let width = input.get(0).unwrap().len();
    let height = input.len();
    let mut terrain_vec = vec![vec![0u8; width]; height];

    // populate int matrix (2dvec)
    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            terrain_vec[i][j] = ch.to_digit(10).unwrap() as u8;
        }
    }

    // its much simpler than it looks
    let mut cnt = 0u32;
    for (i, line_vec) in terrain_vec.iter().enumerate() {
        for (j, heigh_val) in line_vec.iter().enumerate() {
            if j == 0 {
                if heigh_val >= &terrain_vec[i][j + 1] {
                    continue
                }
            } else if j == width - 1 {
                if heigh_val >= &terrain_vec[i][j - 1] {
                    continue
                }
            } else if heigh_val >= &terrain_vec[i][j + 1] || heigh_val >= &terrain_vec[i][j - 1] {
                continue
            }

            if i == 0 {
                if heigh_val >= &terrain_vec[i + 1][j] {
                    continue
                }
            } else if i == height - 1 {
                if heigh_val >= &terrain_vec[i - 1][j] {
                    continue
                }
            } else if heigh_val >= &terrain_vec[i + 1][j] || heigh_val >= &terrain_vec[i - 1][j] {
                continue
            }

            cnt += *heigh_val as u32 + 1;
        }
    }

    cnt as isize
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 15);
}
