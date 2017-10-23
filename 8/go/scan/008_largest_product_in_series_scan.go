package scan


func findLargestProductInSeries(digits *[1000]byte, start, end int) int64 {
	if (end - start)  < 13 {
		return -1
	}

	largestProduct := int64((*digits)[start])
	for i := 1; i < 13 ; i++ {
		d := int64((*digits)[start + i])
		if d == 1 {
			continue
		}
		largestProduct *= d
	}

	currProduct := largestProduct
	for ii := start + 13; ii < end; ii++ {
		old := int64((*digits)[ii-13])
		cur := int64((*digits)[ii])
		if old == cur {
			continue
		}

		if cur == 1 {
			currProduct /= old
			continue
		}

		if old == 1 {
			currProduct *= cur
		} else {
			currProduct = currProduct / old * cur
		}

		if currProduct > largestProduct {
			largestProduct = currProduct
		}
	}

	return largestProduct
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
	var largestProduct int64
	for ii := 0; ii < len(digits) - 13; ii++ {
		if findStart {
			if digits[ii] == 0 {
				continue
			} else {
				start = ii
				findStart = false
			}
		}

		if digits[ii] == 0 {
			end = ii
			result := findLargestProductInSeries(&digits, start, end)
			if result > largestProduct {
				largestProduct = result
			}
			findStart = true
		}
	}

	return largestProduct
}
