use std::collections::HashMap;

fn main() {
    let nums = vec![1, 3, 5, 7, 11, 26, 4, 2, 26];

    let a = avg(&nums);

    let med = median(&nums);

    let m = mode(&nums);

    println!("{a} {med} {m}")
}

fn avg(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for num in v {
        sum += num;
    }
    sum as f64 / v.len() as f64
}

fn median(v: &Vec<i32>) -> i32 {
    let med_idx = v.len() / 2;
    let mut sorted_vec = v.to_vec();
    sorted_vec.sort();
    sorted_vec[med_idx]
}

fn mode(v: &Vec<i32>) -> i32 {
    let mut counts = HashMap::new();

    for num in v {
        let count = counts.entry(num).or_insert(0);
        *count += 1;
    }
    let mut max = 0;
    let mut max_key = 0;
    for (key, value) in &counts {
        if value > &max {
            max = *value;
            max_key = **key;
        }
    }
    max_key
}
