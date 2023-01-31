// This program is a solution to [Project Euler Problem 62](https://projecteuler.net/problem=62).
//
// The problem asks for the smallest cube for which exactly five permutations of its digits are
// also cubes. To solve this problem, a HashMap is used to store the permutation of each cube as
// the key, and the cubes themselves as the values. A loop is used to iterate through the cubes,
// generating each cube and sorting its digits to get a uniform representation for all permutations.
// If a HashMap entry with the same key already exists, it means the new cube is a permutation of
// the same digits and is added to the list of values for that entry. If the list of values for a
// given key is of length 5, the answer is found. The program then prints the first cube in the
// list (which is the smallest cube for which five permutations of its digits are also cubes).

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
            println!("answer: {}", v[0]);
            return
        }
        n += 1;
    }
}
