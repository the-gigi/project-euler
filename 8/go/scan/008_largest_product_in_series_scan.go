package scan

import (
	"strings"
)


func findLargestSumInSeries(digits *[1000]byte, start, end int) (int, int) {
	if (end - start)  < 13 {
		return -1, -1
	}

	largestSum := int((*digits)[start])
	for i := 1; i < 13 ; i++ {
		largestSum += int((*digits)[i])
	}

	currSum := largestSum
	index := 0
	old := int((*digits)[start])
	for i := start + 13; i < end; i++ {
		cur := int((*digits)[i])
		currSum += cur - old
		if currSum > largestSum {
			largestSum = currSum
			index = i
		}
		old = cur
	}

	return largestSum, index
}


func FindLargestProduct(text string) int64 {
	var digits [1000]byte
	digIndex := 0
	for _, c := range text {
		if c == 10 {
			continue
		}
		digits[digIndex] = byte(c) - 48
		digIndex++
	}

	start := -1
	end := -1
	findStart := true
	largestSum := -1
	largestIndex := -1
	for i := 0; i < len(digits) - 13; i++ {
		if findStart {
			if digits[i] == 0 {
				continue
			} else {
				start = i
				findStart = false
			}
		}

		if digits[i] == 0 {
			end = i
			result, index := findLargestSumInSeries(&digits, start, end)
			if result > largestSum {
				largestSum, largestIndex = result, index
			}
			findStart = true
		}
	}

	largestProduct := int64(digits[largestIndex])
	for i := largestIndex + 1; i < largestIndex + 13; i++ {
		largestProduct *= int64(digits[i])
	}

	return largestProduct
}
