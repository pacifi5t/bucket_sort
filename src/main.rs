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
        numbers.push(rng.gen_range(0..u16::MAX as usize));
    }

    numbers
}

fn benchmark(size: usize) -> (Vec<u128>, u128) {
    let mut results = Vec::<u128>::new();
    for _i in 0..3 {
        let input = generate(size);
        let length = input.len();

        let now = Instant::now();
        bucket_sort(input, length / 5);
        results.push(now.elapsed().as_millis());
    }
    let sum: u128 = results.iter().sum();
    let avg = sum / 3;
    println!(
        "Results of 3 runs with size {}: {:?}; Avg: {}",
        size, results, avg
    );

    (results, avg)
}

fn prepare_file(path: &str) -> io::Result<File> {
    let mut file = File::create(path)?;
    writeln!(file, "n\t\ta1\ta2\ta3\tavg\t")?;
    Ok(file)
}

fn export_to_file(file: &mut File, size: usize, results: Vec<u128>, avg: u128) -> io::Result<()> {
    writeln!(
        file,
        "{}\t\t{}\t{}\t{}\t{}",
        size, results[0], results[1], results[2], avg
    )?;
    Ok(())
}

fn main() {
    let sizes: Vec<usize> = vec![1000, 10000, 100000, 1000000, 10000000];
    let mut file:File;

    match prepare_file("./output.txt") {
        Err(why) => panic!("{}", why),
        Ok(created_file) => file = created_file
    }

    for i in 0..sizes.len() {
        let size = sizes[i];
        let (res, avg) = benchmark(size);
        match export_to_file(&mut file, size, res, avg) {
            Err(why) => panic!("{}", why),
            Ok(()) => {}
        }
    }
}
