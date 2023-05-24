fn main() {
    let mut x: i32 = 0;

    for n in 1..1000 {
        if n % 3 == 0 || n % 5 == 0 {
            x = x + n;
        }
    }
    println!("{}", x);
}
