pub fn bucket_sort(array: Vec<usize>, bucket_number: usize) -> Vec<usize> {
    let mut buckets: Vec<Vec<usize>> = Vec::new();
    for _i in 0..bucket_number {
        buckets.push(Vec::<usize>::new());
    }
    let max = array.iter().max().unwrap();

    for each in array.iter() {
        buckets[(each * (bucket_number - 1)) / max].push(each.clone());
    }

    for each in buckets.iter_mut() {
        next_sort(each);
    }

    buckets.concat()
}

fn next_sort(array: &mut Vec<usize>) {
    for i in 1..array.len() {
        let current = array[i];
        let mut j = i - 1;
        while j > 0 && array[j] > current {
            array[j + 1] = array[j];
            j -= 1;
        }
        array[j+1] = current;
    }
}
