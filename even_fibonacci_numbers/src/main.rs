fn main() {
    println!("{}", fibo(0, 1));
}

fn fibo(x: i32, y: i32) -> i32 {
    if x + y >= 4000000 { return 0; }
    let res = fibo(y, x + y);
    if (x + y) % 2 == 0 {
        res + x + y
    } else {
        return res;
    }
}
