use problem_0014::*;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn solution() {
    let input = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    solution::process(divan::black_box(input)).unwrap();
}
