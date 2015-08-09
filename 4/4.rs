
fn is_palindrome(num: u32) -> bool {
    let mut r = 0;
    let mut n = num;

    while n > 0 {
        r = r * 10 + (n % 10);
        n /= 10;
    }

    return num == r;
}

fn main() {
    let mut max = 0;
    for a in 100..999 {
        for b in 100..999 {           
            let x = (1099 - a) * (1099 - b);
            if x > max && is_palindrome(x) {
                max = x;
            }
        }   
    }

    println!("Largest palindrom is: {}", max);
}
