use problem_0003::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn solution() {
    let input = "abcabcbb".to_string();
    solution::process(divan::black_box(input)).unwrap();
}
