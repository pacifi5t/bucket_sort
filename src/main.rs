use rand::Rng;
use std::{
    fs::File,
    io::{self, Write},
    time::Instant,
};

use bucket_sort::bucket_sort;

mod bucket_sort;

fn generate(number: usize) -> Vec<usize> {
    let mut rng = rand::thread_rng();
    let mut numbers = Vec::<usize>::new();
    for _i in 0..number {
        numbers.push(rng.gen_range(0..u32::MAX as usize));
    }

    numbers
}

fn benchmark(size: usize) -> u128 {
    let mut results = Vec::<u128>::new();
    for _i in 0..10 {
        let input = generate(size);
        let length = input.len();

        let now = Instant::now();
        bucket_sort(input, length / 50);
        results.push(now.elapsed().as_micros());
    }
    let avg = results.iter().sum::<u128>() / 10;
    println!(
        "Results of 10 runs with n={}: {:?}; Avg: {}",
        size, results, avg
    );

    avg
}

fn prepare_file(path: &str) -> io::Result<File> {
    let mut file = File::create(path)?;
    writeln!(file, "n\t\tavg")?;
    Ok(file)
}

fn export_to_file(file: &mut File, size: usize, avg: u128) -> io::Result<()> {
    writeln!(file, "{}\t\t{}", size, avg)?;
    Ok(())
}

fn main() {
    let sizes: Vec<usize> = vec![10000, 100000, 1000000];
    let mut file: File;

    match prepare_file("./output.txt") {
        Err(why) => panic!("{}", why),
        Ok(created_file) => file = created_file,
    }

    for i in 0..sizes.len() {
        let size = sizes[i];
        let avg = benchmark(size);
        match export_to_file(&mut file, size, avg) {
            Err(why) => panic!("{}", why),
            Ok(()) => {}
        }
    }
}
