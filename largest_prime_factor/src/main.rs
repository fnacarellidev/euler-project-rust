fn main() {
    println!("{}", largest_prime_factor(600851475143));
}

fn largest_prime_factor(x: i64) -> f64 {
    let mut largest: f64 = -1.0;
    let mut x_copy:f64 = x.clone() as f64;
    let upper_limit:i64 = (x as f64).sqrt() as i64;

    while x_copy % 2.0 == 0.0 {
        largest = 2.0;
        x_copy /= 2.0;
    }
    while x_copy % 3.0 == 0.0 {
        largest = 3.0;
        x_copy /= 3.0;
    }
    for i in (5..upper_limit).step_by(6) {
        while x_copy % i as f64 == 0.0 {
            largest = i as f64;
            x_copy = x_copy / i as f64;
        }
        while x_copy % (i + 2) as f64 == 0.0 {
            largest = (i + 2) as f64;
            x_copy = x_copy / (i + 2) as f64;
        }
    }
    largest
}
