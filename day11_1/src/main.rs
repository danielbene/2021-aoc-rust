use aoc_util;

const TOTAL_STEPS: u8 = 100;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

// omg these dumbo octopuses are soooo cute
fn solve(input: &mut Vec<String>) -> isize {
    let width= input.get(0).unwrap().len();
    let height = input.len();
    let mut flashes: usize = 0;
    let mut octo_matrix = vec![vec![0u8; width]; height];

    for (i, line) in input.iter().enumerate() {
        for (j, ch) in line.chars().enumerate() {
            octo_matrix[i][j] = ch.to_digit(10).unwrap() as u8;
        }
    }

    for step in 0..5 {
        println!("step num: {}", step);
        for line in octo_matrix.iter() {
            println!("{:?}", line);
        }

        println!("--------------------------------");
        let mut after_flash: Vec<(usize, usize)> = Vec::new();
        for w in 0..width {
            for h in 0..height {
                octo_matrix[w as usize][h as usize] += 1;
                if octo_matrix[w as usize][h as usize] == 10 {
                    after_flash.push((w, h));
                }
            }
        }

        /*println!("step num: {} - before flash", step);
        for line in octo_matrix.iter() {
            println!("{:?}", line);
        }

        println!("--------------------------------");*/

        for coord in after_flash {
            // im sure there is like 3 things here thats agains "good practices" :D
            flash_octo(coord.0 as i8, coord.1 as i8, &mut octo_matrix, &mut flashes, &(width as i8), &(height as i8));
        }
    }

    0
}

fn flash_octo(i: i8, j: i8, matrix: &mut Vec<Vec<u8>>, flashes: &mut usize, width: &i8, height: &i8) {
    *flashes += 1;
    let mut after_flash: Vec<(i8, i8)> = Vec::new();

    //println!("start {}, {}", i, j);
    for i2 in i-1..=i+1 {
        for j2 in j-1..=j+1 {
            //println!("test {}, {}", i2, j2);
            if i == i2 && j == j2 {
                continue
            } else if i2 < 0 || j2 < 0 {
                continue
            } else if i2 == *width || j2 == *height {
                continue
            }

            matrix[i2 as usize][j2 as usize] += 1;
            //println!("flash increase {}, {}", i2, j2);
            if matrix[i2 as usize][j2 as usize] == 10 {
                after_flash.push((i2, j2));
            }
        }
    }

    for coord in after_flash {
        flash_octo(coord.0, coord.1, matrix, flashes, width, height);
    }

    rest_octos(matrix, width, height);
}

fn rest_octos(matrix: &mut Vec<Vec<u8>>, width: &i8, height: &i8) {
    for w in 0..*width {
        for h in 0..*height {
            if matrix[w as usize][h as usize] > 9 {
                matrix[w as usize][h as usize] = 0;
            }
        }
    }
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 1656);
}
