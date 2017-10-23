package trivial

import (
	"strings"
)

func calcProdcut(series string) int64 {
	digits := make([]int64, len(series))
	for i, c := range series {
		digits[i] = int64(c) - 48
	}

	product := int64(1)
	for i := 0; i < len(digits); i++ {
		product *= digits[i]
	}

	return product
}

func FindLargestProduct(text string) int64 {
	text = strings.Replace(text, "\n", "", -1)
	largestProduct := int64(0)
	for i := 0; i < len(text); i++ {
		end := i + 13
		if end > len(text) {
			end = len(text)
		}
		series := text[i:end]
		result := calcProdcut(series)
		if result > largestProduct {
			largestProduct = result
		}
	}

	return largestProduct
}
