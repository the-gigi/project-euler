/*
Problem 6

The sum of the squares of the first ten natural numbers is,

12 + 22 + ... + 102 = 385
The square of the sum of the first ten natural numbers is,

(1 + 2 + ... + 10)2 = 552 = 3025
Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.

Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

https://projecteuler.net/problem=6
 */
package main

import (
	"fmt"
)

// The makeIntList() function returns an array of consecutive integers
// starting from 1 all the way to the `number` (including the number)
func MakeIntList(number int) []int {
	result := make([]int, number)
	for i := 1; i <= number; i++ {
		result[i - 1] = i
	}
	return result
}

// The squareList() function takes a slice of integers and returns an
// array of the quares of these integers
func SquareList(numbers []int) []int {
	result := make([]int, len(numbers))

	for i, n := range numbers {
		result[i] = n * n
	}

	return result
}

// The sumList() function takes a slice of integers and returns their sum
func SumList(numbers []int) int {
	result := 0
	for _, n := range numbers {
		result += n
	}

	return result
}

// Solve Project Euler #6 - Sum square difference
func Process(number int) int {
	numbers := MakeIntList(number)
	sum := SumList(numbers)
	squares := SquareList(numbers)

	sumOfSquares := SumList(squares)
	squareOfSum := sum * sum

	diff := squareOfSum - sumOfSquares

	return diff
}

func main() {
	result := Process(100)
	fmt.Println(result)
}
