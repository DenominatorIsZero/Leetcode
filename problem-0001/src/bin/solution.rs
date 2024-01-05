use miette::Context;
use problem_0001::solution::process;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let input = [2, 7, 11, 15].to_vec();
    let target = 9;
    let result = process(input, target).context("process input")?;
    println!("{:?}", result);
    Ok(())
}
