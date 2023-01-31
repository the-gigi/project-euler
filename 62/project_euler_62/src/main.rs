// The cube, 41063625 (3453), can be permuted to produce two other cubes: 56623104 (3843) and 66430125 (4053). In fact, 41063625 is the smallest cube which has exactly three permutations of its digits which are also cube.
//
// Find the smallest cube for which exactly five permutations of its digits are cube.
//
// https://projecteuler.net/problem=62

#[macro_use]
extern crate fstrings;

use std::borrow::Borrow;
use std::collections::HashMap;

fn sort_number_digits(n: u64) -> String {
    let mut s = n.to_string().into_bytes();
    s.sort();

    return String::from_utf8(s).unwrap().to_string()
}

fn main() {
    let mut cubes: HashMap<String, Vec<u64>> = HashMap::new();
    let mut n = 1;
    loop {
        let cube: u64 = n * n * n;
        let sorted = sort_number_digits(cube);
        cubes.entry(sorted.clone()).or_default().push(cube);
        let v = &cubes[&sorted];
        if v.len() == 5 {
            for i in v {
                println!("{}", i);
            }
            println!("answer: {n} {}", v[0]);
            return
        }

        n += 1;
    }
}
