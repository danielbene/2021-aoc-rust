use aoc_util;
use petgraph::graph::Graph;

fn main() {
    let mut init_tuple = aoc_util::init();
    let solution = solve(&mut init_tuple.0);
    aoc_util::end(solution as isize, init_tuple.1);
}

// https://depth-first.com/articles/2020/02/03/graphs-in-rust-an-introduction-to-petgraph/
fn solve(input: &mut Vec<String>) -> isize {
    let mut cave_graph = Graph::new();
    for line in input {
        
    }

   0
}

#[test]
fn tests() {
    let mut vec: Vec<String> = aoc_util::load_from_file("test.txt");
    assert!(solve(&mut vec) == 10);
    vec = aoc_util::load_from_file("test2.txt");
    assert!(solve(&mut vec) == 19);
    vec = aoc_util::load_from_file("test3.txt");
    assert!(solve(&mut vec) == 226);
}
