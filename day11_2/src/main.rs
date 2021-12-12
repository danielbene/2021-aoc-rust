use aoc_util;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

// im sure there's at leaset like 3 things here that goes agains "good practices" :,D
// TODO: refactor later - solution is far from optimal
fn solve(input: &mut Vec<String>) -> isize {
    let width = input.get(0).unwrap().len();
    let height = input.len();
    let octo_cnt = (width * height) as u16;
    let mut steps = 0;
    let mut octo_matrix = vec![vec![0i8; width]; height];

    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            octo_matrix[i][j] = ch.to_digit(10).unwrap() as i8;
        }
    }

    loop {
        steps += 1;
        let mut flashes: u16 = 0;
        let mut after_flash: Vec<(i8, i8)> = Vec::new();
        for w in 0..width {
            for h in 0..height {
                octo_matrix[w][h] += 1;
                if octo_matrix[w][h] > 9 {
                    octo_matrix[w][h] = -1;
                    after_flash.push((w as i8, h as i8));
                }
            }
        }

        for coord in after_flash {
            flash_octo(&coord.0, &coord.1, &mut octo_matrix, &mut flashes, &(width as i8), &(height as i8));
        }

        if flashes == octo_cnt {
            break
        }

        rest_octos(&mut octo_matrix, &(width as i8), &(height as i8));
    }

    steps as isize
}

fn flash_octo(i: &i8, j: &i8, matrix: &mut Vec<Vec<i8>>, flashes: &mut u16, width: &i8, height: &i8) {
    *flashes += 1;
    let mut after_flash: Vec<(i8, i8)> = Vec::new();

    for i2 in i-1..=i+1 {
        for j2 in j-1..=j+1 {
            if *i == i2 && *j == j2 {
                continue
            } else if i2 < 0 || j2 < 0 {
                continue
            } else if i2 == *width || j2 == *height {
                continue
            }

            let val: &mut i8 = &mut matrix[i2 as usize][j2 as usize];
            if *val != -1 {
                *val += 1;
            }

            if *val > 9 {
                *val = -1;
                after_flash.push((i2, j2));
            }
        }
    }

    for coord in after_flash {
        flash_octo(&coord.0, &coord.1, matrix, flashes, width, height);
    }
}

fn rest_octos(matrix: &mut Vec<Vec<i8>>, width: &i8, height: &i8) {
    for w in 0..*width {
        for h in 0..*height {
            if matrix[w as usize][h as usize] == -1 {
                matrix[w as usize][h as usize] = 0;
            }
        }
    }
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 195);
}
