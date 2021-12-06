use aoc_util;

const MATRIX_SIZE: usize = 999;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

fn solve(input: &mut Vec<String>) -> isize{
    let mut counter: u64 = 0;
    // 2darray causes stackoverflow on 1k size - use Vec, it is stored on heap...
    // let mut terrain = [[0u16; MATRIX_SIZE]; MATRIX_SIZE];
    let mut terrain_vec = vec![vec![0; MATRIX_SIZE]; MATRIX_SIZE];  // dimension indexing is swapped!
    for line in input {
        let coords: Vec<&str> = line.split(" -> ").collect();
        let s: Vec<&str> = coords.get(0).unwrap().split(",").collect();
        let e: Vec<&str> = coords.get(1).unwrap().split(",").collect();

        let start: (u16, u16) = (s[0].parse().unwrap(), s[1].parse().unwrap());
        let end: (u16, u16) = (e[0].parse().unwrap(), e[1].parse().unwrap());
        if start.0 != end.0 && start.1 == end.1 {
            mtx_job_on_x(start, end, &mut terrain_vec, &mut counter);
        } else if start.0 == end.0 && start.1 != end.1 {
            mtx_job_on_y(start, end, &mut terrain_vec, &mut counter);
        } else {
            mtx_job_diagonal(start, end, &mut terrain_vec, &mut counter);
        }
    }

    counter as isize
}

fn mtx_job_on_x(start: (u16, u16), end: (u16, u16), array: &mut Vec<Vec<u16>>, cnt: &mut u64) {
    let mut range = end.0..=start.0;
    let fix = start.1;
    if start.0 < end.0 {
        range = start.0..=end.0;
    }

    for i in range {
        array[fix as usize][i as usize] += 1;
        if array[fix as usize][i as usize] == 2 {
            *cnt += 1;
        }
    }
}

fn mtx_job_on_y(start: (u16, u16), end: (u16, u16), array: &mut Vec<Vec<u16>>, cnt: &mut u64) {
    let mut range = end.1..=start.1;
    let fix = start.0;
    if start.1 < end.1 {
        range = start.1..=end.1;
    }

    for i in range {
        array[i as usize][fix as usize] += 1;
        if array[i as usize][fix as usize] == 2 {
            *cnt += 1;
        }
    }
}

fn mtx_job_diagonal(start: (u16, u16), end: (u16, u16), array: &mut Vec<Vec<u16>>, cnt: &mut u64) {
    // Range and Rev not compatible (and ranges only increment) so manually building the steps
    // am i missing something or is it really fcked up?
    let mut x_steps: Vec<u16> = Vec::new();
    if start.0 > end.0 {
        for i in (end.0..=start.0).rev() {
            x_steps.push(i);
        }
    } else {
        for i in start.0..=end.0 {
            x_steps.push(i);
        }
    }

    let mut y_steps: Vec<u16> = Vec::new();
    if start.1 > end.1 {
        for i in (end.1..=start.1).rev() {
            y_steps.push(i);
        }
    } else {
        for i in start.1..=end.1 {
            y_steps.push(i);
        }
    }

    for (i, x) in x_steps.iter().enumerate() {
        let y = y_steps.get(i).unwrap();
        array[*y as usize][*x as usize] += 1;
        if array[*y as usize][*x as usize] == 2 {
            *cnt += 1;
        }
    }
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 12);
}
