use problem_0002::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn solution() {
    solution::process(divan::black_box(include_str!(
        "../input.txt",
    )))
    .unwrap();
}