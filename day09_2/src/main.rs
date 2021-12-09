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

    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            terrain_vec[i][j] = ch.to_digit(10).unwrap() as u8;
        }
    }

    let mut cnt_vec: Vec<u32> = Vec::new();
    let mut mirror = terrain_vec.to_owned();
    for (i, line_vec) in terrain_vec.iter().enumerate() {
        for (j, _) in line_vec.iter().enumerate() {
            let mut cnt = 0u32;
            rec_search(i, j, &mut cnt, height, width, &terrain_vec, &mut mirror);
            if cnt != 0 {
                cnt_vec.push(cnt);
            }
        }
    }

    cnt_vec.sort();
    cnt_vec.reverse();
    let mut basin_multi = 1;
    for i in 0..3 {
        basin_multi *= cnt_vec[i];
    }

    basin_multi as isize
}

// i hate this
fn rec_search(i: usize, j: usize, cnt: &mut u32, height: usize, width: usize, terrain: &Vec<Vec<u8>>, mirror: &mut Vec<Vec<u8>>) -> u32 {
    if terrain[i][j] == 9 || mirror[i][j] == 9 {
        return *cnt
    }

    *cnt += 1;
    mirror[i][j] = 9;

    if i != height - 1 {
        *cnt = rec_search(i + 1, j, cnt, height, width, terrain, mirror);
    }

    if i != 0 {
        *cnt = rec_search(i - 1, j, cnt, height, width, terrain, mirror);
    }

    if j != width - 1 {
        *cnt = rec_search(i, j + 1, cnt, height, width, terrain, mirror);
    }

    if j != 0 {
        *cnt = rec_search(i, j - 1, cnt, height, width, terrain, mirror);
    }

    *cnt
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 1134);
}
