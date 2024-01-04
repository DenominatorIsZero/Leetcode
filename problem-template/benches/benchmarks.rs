use {{crate_name}}::*;

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