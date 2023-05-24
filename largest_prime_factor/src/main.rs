fn main() {
    println!("{}", largest_prime_factor(600851475143));
}

fn largest_prime_factor(x: i64) -> f64 {
    let mut largest: f64 = -1.0;
    let mut x_copy:f64 = x.clone() as f64;
    let x_start_value:f64 = x.clone() as f64;

    while x_copy % 2.0 == 0.0 {
        largest = 2.0;
        x_copy /= 2.0;
    }

    while x_copy % 3.0 == 0.0 {
        largest = 3.0;
        x_copy /= 3.0;
    }
    let mut i: f64 = 5.0;
    while i <= x_start_value.sqrt()
    {
        while x_copy % i == 0.0 {
            largest = i;
            x_copy = x_copy / i;
        }
        while x_copy % (i + 2.0) == 0.0 {
            largest = i + 2.0;
            x_copy = x_copy / (i + 2.0);
        }
        i += 6.0;
    }
    largest
}
