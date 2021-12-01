use std::{
    fs,
    time::Instant,
    io::BufReader,
    io::BufRead
};

fn main() {
    let mut lines_vec = load_from_file("input.txt");

    let start = Instant::now();
    let solution = solve(&mut lines_vec);
    let runtime = Instant::now() - start;

    let output = format!("Solution: {}\nRuntime: {}ms ({} microsecs)\n",
        solution, runtime.as_millis(), runtime.as_micros());
    fs::write("solution.txt", output).expect("Unable to write file");
}

fn solve(input: &mut Vec<String>) -> i16 {
    let mut counter: i16 = -1;
    let mut prev: i16 = 0;
    let item_cnt = input.len();

    let dif = item_cnt % 3;
    if dif != 0 {
        println!("dif is {}", dif);
        for i in 1..=dif {
            input.remove(item_cnt - i);
            println!("removing item in index {}", item_cnt - i);
        }
    }

    // TODO: implement sliding

    for triplet in 0..item_cnt/3 {
        let mut current: i16 = 0;
        for i in 0..3 {
            let index = triplet * 3 + i;
            current += input.get(index).unwrap().parse::<i16>().unwrap();
        }

        println!("prev: {}, curr: {}", prev, current);

        if current > prev {
            counter += 1;
        }

        prev = current;
    }

    println!("counter {}", counter);

    return counter;
}

fn load_from_file(file_path: &str) ->  Vec<String> {
    let file = fs::File::open(file_path).expect("Something went wrong reading the file");
    let reader = BufReader::new(file);

    reader.lines()
        .map(|line| line.unwrap().parse::<String>().unwrap())
        .collect()
}

#[test]
fn tests() {
    let test_input = "199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n";
    let mut vec: Vec<String> = test_input.lines()
        .map(|line| line.parse::<String>().unwrap())
        .collect();
    assert!(solve(&mut vec) == 5);
}
