fn main() {
    println!("{}", largest_palindrome_product());
}

fn largest_palindrome_product() -> i32 {
    let mut product;
    let mut product_as_str;
    let mut max = 0;

    for i in (100..=999).rev() {
        for j in (100..=999).rev() {
            product = i * j;
            product_as_str = product.to_string();
            if is_palindrome(product_as_str) && product > max {
                max = product;
            }
        }
    }
    max
}

fn is_palindrome(str: String) -> bool {
    let str_bytes: &[u8] = str.as_bytes();

    for i in 0..=(str.len() / 2) {
        if str_bytes[i] != str_bytes[str.len() - 1 - i] {
            return false;
        }
    }
    true
}
