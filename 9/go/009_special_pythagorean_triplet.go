package main

import (
	"fmt"
)

func findSpecialPythagoreanTriplet() (int, int, int) {
	for a := 1; a < 1000; a++ {
		for b := a; b+a < 1000; b++ {
			c := 1000 - b - a
			if a*a+b*b == c*c {
				return a, b, c
			}
		}
	}
	return -1, -1, -1
}

func main() {
	a, b, c := findSpecialPythagoreanTriplet()
	fmt.Println("Triplet:", a, b, c)
	fmt.Printf("a^2: %d, b^2: %d, c^2: %d\n", a*a, b*b, c*c)
	fmt.Printf("Product: %d\n", a*b*c)
}
