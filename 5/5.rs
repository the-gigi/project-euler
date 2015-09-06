/// Smallest Multiple - https://projecteuler.net/problem=5
///
/// 2520 is the smallest number that can be divided by each of the numbers from
/// 1 to 10 without any remainder.
///
///What is the smallest positive number that is evenly divisible by all of the
///numbers from 1 to 20?

use std::collections::HashMap;
//use std::collections::Vec;


/// Return a dictionary with prime keys and exponent values
///
/// The product of all the keys raised to their exponent is the number
///
/// # Example:
///
/// 20 = 2 ** 2 * 5 -> {2: 2, 5:1}
///
fn get_prime_product(n: u64) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();
    let primes_to_20 = [2, 3, 5, 7, 11, 13, 17, 19];

    // Handle quickly case where the number is already prime
    let found = primes_to_20.iter().find(|&x| *x == n);
    match found {
        Some(_) => {
            result.insert(n, 1);
            return result;
        }
        None => {
        }
    }

    // Initialize all multiples to primes to 0
    for p in primes_to_20.iter() {
            result.insert(*p, 0);
    }

    let mut number: u64 = n;
    for p in primes_to_20.iter() {
        while number % *p == 0 {
            let x = result[p].clone();
            result.insert(*p, x + 1);
            number /= *p;
        }
    }

    result = result.iter().filter(|&(_, v)| *v != 0)
                          .map(|(&k, &v)| (k, v))
                          .collect();
    return result;
}

fn main() {
    let mut result: HashMap<u64, u64> = HashMap::new();

    for n in 0..19 {
        let prime_product = get_prime_product(20 - n);
        for (&k, &v) in prime_product.iter() {
            if !result.contains_key(&k) || v > result[&k] {
                result.insert(k, v);
            }
        }
    }

    let mut final_number = 1;
    for (&k, &v) in result.iter() {
        final_number *= k.pow(v as u32);
    }

    println!("{}", final_number);
    assert_eq!(232792560, final_number)
}
