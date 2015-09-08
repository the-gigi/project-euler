/// Largest Prime Factor
///
/// The prime factors of 13195 are 5, 7, 13 and 29.
///
/// What is the largest prime factor of the number 600851475143 ?

use std::collections::HashMap;

fn find_next_prime(primes: &Vec<u64>) -> u64 {
    let last = primes[primes.len() - 1];
    let mut candidates = HashMap::new();

    let start: u64 = last + 1;
    let end: u64 = last * 2 + 1;

    for p in start..end {
        candidates.insert(p, 1);
    }

    for p in primes.iter() {
        let mut i = *p * 2;
        while i < end {
            candidates.insert(i, 0);
            i += *p;
        }
    }

    for i in start..end {
        let value = candidates[&i];
        if value == 1 {
            return i;
        }
    }

    return 0;
}

//fn int_vec_to_string(v: &Vec<u64>) -> String {
//    let d: Vec<String> = v.iter().map(|&x| x.to_string()).collect();
//    return d.connect(", ");
//}

fn find_largest_prime_factor(number: u64) -> u64 {
    let mut largest = 1;
    let mut primes = vec![2];
    let mut num = number;

    loop {
        let last = primes[primes.len() - 1];
        if num % last == 0 {
            largest = last;
            println!("{}", largest);
            num /= last;
        }

        // Keep dividing
        while num % last == 0 {
            num /= last
        }

        if num == 1 {
            return largest;
        }

        let next_prime = find_next_prime(&primes);
        primes.push(next_prime);
    }
}

fn main() {
    let number = 600851475143;
    let largest = find_largest_prime_factor(number);
    println!("Largest prime factor of {} is {}", number, largest);
    assert_eq!(6857, larget);
}


//#[test]
//fn test_find_next_prime() {
//    let mut primes = vec![2];
//    let mut p = find_next_prime(&primes);
//    assert_eq!(3, p);
//
//
//    primes = vec![2, 3];
//    p = find_next_prime(&primes);
//    assert_eq!(5, p);
//
//    primes = vec![2, 3, 5, 7];
//    p = find_next_prime(&primes);
//    assert_eq!(11, p);
//}
