use took::Timer;

fn main() {
    let timer = Timer::new();
    runner::jobs().iter().for_each(|j| j.0());
    timer.took().describe("everything");
}
