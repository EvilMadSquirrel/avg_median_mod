fn main() {
    let nums = vec![1, 3, 5, 7, 11, 26, 13, 4];

    let a = avg(&nums);

    println!("{a}")
}

fn avg(v: &Vec<i32>) -> f64 {
    let mut sum = 0;
    for num in v {
        sum += num;
    }
    sum as f64/v.len() as f64
}

// fn median() {}

// fn mode() {}
