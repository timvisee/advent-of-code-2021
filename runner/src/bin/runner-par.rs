use rayon::prelude::*;
use took::Timer;

fn main() {
    // Build threadpool with larger stack size
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .stack_size(12_800_000)
        .build_global()
        .unwrap();

    let jobs = runner::jobs();
    let timer = Timer::new();
    (0..jobs.len()).into_par_iter().for_each(|i| jobs[i].0());
    timer.took().describe("everything");
}
