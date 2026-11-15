use std::fmt::Display;

/// Solves and measures the execution time of the given function.
pub fn solve<T: Display>(f: impl Fn(&str) -> Option<T>, input: &str, part: u8) {
    let timer = std::time::Instant::now();
    let result = f(input);
    let elapsed_us = timer.elapsed().as_micros();

    println!(
        "Part {}: \x1b[1m{}\x1b[0m ({:.2}µs)",
        part,
        result.map(|v| v.to_string()).unwrap_or("✖ ".to_string()),
        elapsed_us
    );
}
