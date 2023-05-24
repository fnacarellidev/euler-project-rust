fn main() {
    let mut current = 1;
    let mut previous = 0;
    let mut total = 0;
    let mut aux;

    loop {
        aux = current + previous;
        previous = current;
        current = aux;
        if aux % 2 == 0 { total += aux; }
        if aux >= 4000000 { break; }
    } 
    println!("{}", total);
}

// fn fibo(x: i32, y: i32) -> i32 {
//     if x + y >= 13 { return 0; }
//     fibo(y, x + y);
//     if x + y % 2 == 0 { x + y }
//     else { return 0; }
// }
