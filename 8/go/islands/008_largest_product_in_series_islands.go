package islands

import (
	"strings"
)

func findLargestProdcutInSeries(input string) int64 {
	digits := make([]int, len(input))
	for i, c := range input {
		digits[i] = int(c) - 48
	}

	maxLength := 13
	if len(input) < 13 {
		maxLength = len(input)
	}

	largestSum := 0
	for i := 0; i < maxLength; i++ {
		largestSum += digits[i]
	}

	currSum := largestSum
	index := 0
	for i := maxLength; i < len(digits); i++ {
		d := digits[i]
		currSum += d - digits[index]
		if currSum > largestSum {
			largestSum = currSum
		}

		index++
	}

	result := int64(digits[index])
	for i := index + 1; i < index+maxLength; i++ {
		result *= int64(digits[i])
	}
	return result
}


func FindLargestProduct(text string) int64 {
	text = strings.Replace(text, "\n", "", -1)
	islands := strings.Split(text, "0")
	largestProduct := int64(0)
	for _, island := range islands {
		if island == "" {
			continue
		}
		result := findLargestProdcutInSeries(island)
		if result > largestProduct {
			largestProduct = result
		}
	}

	return largestProduct
}
