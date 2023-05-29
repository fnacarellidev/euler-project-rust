fn main() {
    println!("{}", get_square_of_sum_of(100) - get_sum_of_squares_of(100));
}

fn get_sum_of_squares_of(mut i: i32) -> i32 {
    let mut sum_of_squares: i32 = 0;

    while i > 0 {
        sum_of_squares += i.pow(2);
        i -= 1;
    }
    sum_of_squares
}

fn get_square_of_sum_of(mut i: i32) -> i32 {
    let mut sum = 0;

    while i > 0 {
        sum += i;
        i -= 1;
    }
    sum.pow(2)
}
