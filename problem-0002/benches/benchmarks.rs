use problem_0002::{solution::ListNode, *};

fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench]
fn solution() {
    let l1 = ListNode::from_vec(vec![2, 4, 3]);
    let l2 = ListNode::from_vec(vec![5, 6, 4]);
    solution::process(divan::black_box(l1), divan::black_box(l2)).unwrap();
}
