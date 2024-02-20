use miette::Context;
use problem_0002::solution::process;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let l1 = None;
    let l2 = None;

    let result = process(l1, l2).context("process input")?.unwrap();
    println!("{:?}", result);
    Ok(())
}
