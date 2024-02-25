use problem_0009::*;
use rand::Rng;

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn solution() {
    let mut rng = rand::thread_rng();

    for _ in 0..1000 {
        let input = rng.gen_range(0..1000000);
    solution::process(divan::black_box(input))
    .unwrap();
    }
}