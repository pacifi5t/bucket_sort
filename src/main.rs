use std::time::Instant;

use rand::Rng;

use bucket_sort::bucket_sort;

mod bucket_sort;

fn generate(number: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut numbers = Vec::<usize>::new();
    for _i in 0..number {
        numbers.push(rng.gen_range(0..u16::MAX as usize));
    }

    numbers
}

fn benchmark(size: usize) -> Vec<u128> {
    let mut durations = Vec::<u128>::new();
    for _i in 0..3 {
        let input = generate(size);
        let length = input.len();

        let now = Instant::now();
        bucket_sort(input, length / 5);
        durations.push(now.elapsed().as_millis());
    }
    let sum: u128 = durations.iter().sum();
    println!(
        "Results of 3 runs with size {}: {:?}; Avg: {}",
        size,
        durations,
        sum / 3
    );

    durations
}

fn main() {
    benchmark(100000);
    benchmark(1000000);
    benchmark(10000000);
    //TODO: Export results to file
}
