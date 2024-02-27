use miette::Context;
use problem_0014::solution::process;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

#[tracing::instrument]
fn main() -> miette::Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    #[cfg(not(feature = "dhat-heap"))]
    tracing_subscriber::fmt::init();

    let input = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];
    let result = process(input).context("process input")?;
    println!("{}", result);
    Ok(())
}
