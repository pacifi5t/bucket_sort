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

fn main() {
    let input = generate(30);
    let sorted = bucket_sort(input, 5);
    println!("Sorted: {:?}", sorted);
}
