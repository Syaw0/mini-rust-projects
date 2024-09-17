/* Task:
Given a list of integers, use a vector and return the median 
(when sorted, the value in the middle position) 
and mode (the value that occurs most often; a hash map will 
be helpful here) of the list.
*/

use std::collections::HashMap;

fn main() {
    calculate_media_mode(&[-15, 30, 100, 23, -1, 1, 34, 100, 0, 1, 11]);
}

fn calculate_media_mode(numbers: &[i32]) {
    if numbers.len() < 2 {
        println!("We need at least two or more numbers");
        return;
    }
    let mut sorted_numbers = Vec::from(numbers);
    sorted_numbers.sort();
    println!("{sorted_numbers:?}");

    let median = match sorted_numbers.len() % 2 {
        0 => {
            let median_index = sorted_numbers.len() / 2;
            let med1 = sorted_numbers[median_index - 1] as f64;
            let med2 = sorted_numbers[median_index] as f64;
            (med1 + med2) / 2.0
        }
        _ => { sorted_numbers[sorted_numbers.len() / 2] as f64 }
    };

    //
    let mut counter = HashMap::new();
    for i in &sorted_numbers {
        let number = counter.entry(i).or_insert(0);
        *number += 1;
    }

    let mode = counter
        .into_iter()
        .max_by_key(|&(key, _)| key)
        .map(|(num, _)| num)
        .unwrap();

    println!("median:{median} mode:{mode}");
}
