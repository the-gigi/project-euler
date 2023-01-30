// Problem 17
//
// If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are
// 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
//
// If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words,
// how many letters would be used?
//
// NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23
// letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out
// numbers is in compliance with British usage.
//
// Link: https://projecteuler.net/problem=17

use lazy_static::lazy_static;
use std::collections::HashMap;

const ZERO: usize = 0;

lazy_static! {
    static ref SINGLES: HashMap<usize, usize> = {
        let m = vec![
            (1, "one".len()),
            (2, "two".len()),
            (3, "three".len()),
            (4, "four".len()),
            (5, "five".len()),
            (6, "six".len()),
            (7, "seven".len()),
            (8, "eight".len()),
            (9, "nine".len())].into_iter()
                              .collect();
        m
    };

    static ref TENS: HashMap<usize, usize> = {
        let m = vec![
            (2, "twenty".len()),
            (3, "thirty".len()),
            (4, "forty".len()),
            (5, "fifty".len()),
            (6, "sixty".len()),
            (7, "seventy".len()),
            (8, "eighty".len()),
            (9, "ninety".len()),].into_iter()
                                .collect::<HashMap<_, _>>();
        m
    };

    static ref TEENS: HashMap<usize, usize> = {
        let m = vec![
            (10, "ten".len()),
            (11, "eleven".len()),
            (12, "twelve".len()),
            (13, "thirteen".len()),
            (14, "fourteen".len()),
            (15, "fifteen".len()),
            (16, "sixteen".len()),
            (17, "seventeen".len()),
            (18, "eighteen".len()),
            (19, "nineteen".len()),].into_iter()
                                .collect::<HashMap<_, _>>();
        m
    };

}

fn singles_to_char_count(digit: usize) -> &'static usize {
    return SINGLES.get(&digit).unwrap_or(&ZERO);
}

fn tens_to_char_count(digit: usize) -> &'static usize {
    return TENS.get(&digit).unwrap_or(&ZERO);
}

fn teens_to_char_count(number: usize) -> &'static usize {
    return TEENS.get(&number).expect("invalid teen");
}

fn hundreds_to_char_count(digit: usize) -> usize {
    if digit == 0 {
        return 0;
    }
    return singles_to_char_count(digit) + "hundred".len()
}


fn number_to_char_count(number: usize) -> usize {
    let hundreds: usize = &number / 100;
    let remainder: usize = &number % 100;
    let tens: usize = &number % 100 / 10;
    let singles: usize = &number % 10;

    let number_has_and = hundreds > 0 && (tens > 0 || singles > 0);
    let mut result = hundreds_to_char_count(hundreds);
    if (remainder > 9) && (remainder < 20) {
        result += teens_to_char_count(tens * 10 + singles)
    } else {
        result += tens_to_char_count(tens);
        result += singles_to_char_count(singles);
    }

    if number_has_and {
        result += 3; // for the "and"
    }

    result
}

fn main() {
    let mut result: usize = 0;
    for i in 1..1000 {
        let count = number_to_char_count(i as usize);
        result += count
    }

    // Add "one thousand" with no space
    result += "onethousand".len();
    println!("result: {result}")
}