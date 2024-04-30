fn is_prime(n: u32) -> bool {
    if n < 2 { return false; }
    if n == 2 || n == 3 { return true; }
    if n % 2 == 0 || n % 3 == 0 { return false; }

    let mut i = 5;

    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 { return false; }
        i += 6;
    }

    true
}

fn find_prime_n(n: u32) -> u32 {
    let mut i = 2;
    let mut prime_vec: Vec<u32> = Vec::new();

    while prime_vec.len() < n.try_into().unwrap() {
        if is_prime(i) {
            prime_vec.push(i);
        }
        i += 1;
    }
    let idx: usize = n.try_into().expect("couldn't cast to usize");
    prime_vec[idx - 1]
}

fn main() {
    println!("{}", find_prime_n(10001));
}
