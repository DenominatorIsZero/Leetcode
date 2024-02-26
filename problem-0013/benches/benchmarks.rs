use problem_0013::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn solution() {
    solution::process(divan::black_box("III".to_string())).unwrap();
}
