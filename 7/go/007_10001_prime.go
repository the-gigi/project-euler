package main

import (
	"fmt"
	"os/exec"
	"strconv"
	"strings"
)

func getCPUCount() (int, error) {
	// sysctl hw.ncpu returns "hw.ncpu: 8\n"
	out, err := exec.Command("sysctl", "hw.ncpu").CombinedOutput()
	if err != nil {
		return -1, err
	}

	s := strings.Fields(string(out))[1]

	result, err := strconv.Atoi(s)
	if err != nil {
		return -1, err
	}
	return result, nil
}

func discoverNewPrime(start int, candidates []bool) int {
	for i := 0 + 1; i < len(candidates); i++ {
		if candidates[i] {
			return start + i
		}
	}

	return -1
}

func makeCandidateSlice(start int, end int, primes []int) []bool {
	candidates := make([]bool, end-start+1)
	lastPrime := primes[len(primes)-1]
	for i := start; i <= end; i++ {
		candidates[i-start] = i > lastPrime
	}

	for _, p := range (primes) {
		if p >= start && p <= end {
			candidates[p-start] = true
		}
	}

	return candidates
}

func findPrimes(start int, end int, primes []int) []int {
	lastPrime := primes[len(primes)-1]
	candidates := makeCandidateSlice(start, end, primes)
	for i := 0; i < len(primes); i++ {
		p := primes[i]
		c := p
		offset := start % p
		if offset > 0 {
			c = p - offset
		} else {
			c = 0
		}

		for c <= end-start {
			candidates[c] = false
			c += p
		}

		// Last prime
		if i == len(primes)-1 {
			// Discover new prime in range
			lastPrime = discoverNewPrime(start, candidates)
			if lastPrime != -1 {
				primes = append(primes, lastPrime)
			} else {
				break
			}
		}
	}
	return primes
}

func main() {
	//cpuCount, err := getCPUCount()
	//if err != nil {
	//	fmt.Println("Fail! " + err.Error())
	//}
	//fmt.Printf("CPU count: %d\n", cpuCount)
	primes := []int{2, 3}

	start := 0
	chunkSize := 1000
	for len(primes) < 10001 {
		primes = findPrimes(start, start+chunkSize-1, primes)
		start += chunkSize
	}
	fmt.Printf("The 10001s prime is: %d\n", primes[10000])
}
