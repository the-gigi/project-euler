/*
A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation
of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically,
we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

012   021   102   120   201   210

What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?
 */

package main


import (
	"fmt"
)

var counter int

func perm(s string, nth int) (result []string) {
	if len(s) == 1 {
		return []string{s}
	}

	for i, c := range(s) {
		rest := s[:i] + s[i+1:]
		perms := perm(rest, -1)
		for _, p := range perms {
			if nth != -1 {
				counter += 1
				if counter == nth {
					result = []string{string(c) + p}
					return
				}
			} else {
				result = append(result, string(c)+p)
			}
		}
	}

	return
}

func main() {
	result := perm("0123456789", 1000000)
	fmt.Println(result[0]) // should be: 2783915460
}
