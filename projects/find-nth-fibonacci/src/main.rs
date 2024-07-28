// use std::{}

// ============================================================================

const GOLDEN_RATIO: f64 = 1.618033988749895; // known as φ (phi)

fn main() {
    // just clear the console for us
    print!("\x1b[2J\x1b[1;1H");

    let input = 6;

    let nth_of_input = calculate_nth_fibonacci(input);

    println!("the {input}th of fibonacci sequence is : {nth_of_input}");
}

fn calculate_nth_fibonacci(nth: u32) -> f64 {
    let square_of_five: f64 = (5 as f64).sqrt();

    let nth: f64 = nth.into();

    (GOLDEN_RATIO.powf(nth) - (1.0 - GOLDEN_RATIO).powf(nth)) / square_of_five
}
