package main

import (
	"fmt"
	"math"
)

func findDivisorCount(n int) int {
	divCount := 2 // for 1 and n
	sq := int(math.Sqrt(float64(n)))
	for i := 2; i <= sq; i++ {
		if n%i != 0 {
			continue
		}
		if i == sq && i*i == n {
			divCount++
		} else {
			divCount += 2
		}
	}
	return divCount
}

func findHighlyDivisibleTriangularNumber() int {
	count := 1
	tri := 1
	for findDivisorCount(tri) <= 500 {
		count++
		tri += count
	}

	return tri
}

func main() {
	tri := findHighlyDivisibleTriangularNumber()
	fmt.Println(tri)
}
