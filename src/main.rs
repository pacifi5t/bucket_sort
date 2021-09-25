use rand::Rng;
use std::{
    env,
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

fn benchmark(size: usize, bucket_number: usize) -> u128 {
    println!("n = {}", size);
    let mut results = Vec::<u128>::new();
    for i in 1..11 {
        let array = generate(size);
        let now: Instant;

        if bucket_number == 0 {
            let length = array.len();
            now = Instant::now();
            bucket_sort(array, length);
        } else {
            now = Instant::now();
            bucket_sort(array, bucket_number);
        }

        let elapsed = now.elapsed().as_micros();
        results.push(elapsed);
        println!("Run {}: {}us", i, elapsed);
    }
    let avg = results.iter().sum::<u128>() / 10;
    println!("Average: {}us\n", avg);

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
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Expected at least 2 arguments, got {}", args.len() - 1);
        return;
    }

    let bucket_number;
    let mut sizes = Vec::<usize>::new();

    match args[1].parse::<usize>() {
        Ok(value) => {
            bucket_number = value;
            if bucket_number == 0 {
                println!("k = n\n");
            } else {
                println!("k = {}\n", bucket_number);
            }
            
        }
        Err(why) => panic!("{}", why),
    }

    for i in 2..args.len() {
        match args[i].parse::<usize>() {
            Ok(value) => sizes.push(value),
            Err(why) => panic!("{}", why),
        }
    }

    let mut file: File;

    match prepare_file("./output.txt") {
        Err(why) => panic!("{}", why),
        Ok(created_file) => file = created_file,
    }

    for i in 0..sizes.len() {
        let size = sizes[i];
        let avg = benchmark(size, bucket_number);
        match export_to_file(&mut file, size, avg) {
            Err(why) => panic!("{}", why),
            Ok(()) => {}
        }
    }
}
