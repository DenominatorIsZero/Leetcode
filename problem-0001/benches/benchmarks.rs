use problem_0001::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn solution() {
    let input = [2, 7, 11, 15].to_vec();
    let target = 9;
    solution::process(divan::black_box(input), divan::black_box(target)).unwrap();
}
