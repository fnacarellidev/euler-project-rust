fn main() {
    println!("{}", smallest_multiple());
}

fn smallest_multiple() -> i32 {
    let mut n: i32 = 20;
    let mut start: i32;

    loop {
        start = 20;
        while start > 1 {
            if n % start != 0 {
                break ;
            }
            start -= 1;
        }
        if start == 1 {
            break ;
        }
        n += 1;
    }
    n
}
