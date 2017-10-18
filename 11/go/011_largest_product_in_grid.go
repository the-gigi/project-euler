package main

import (
	"fmt"
	"strconv"
	"strings"
)

const text = `
	08 02 22 97 38 15 00 40 00 75 04 05 07 78 52 12 50 77 91 08
	49 49 99 40 17 81 18 57 60 87 17 40 98 43 69 48 04 56 62 00
	81 49 31 73 55 79 14 29 93 71 40 67 53 88 30 03 49 13 36 65
	52 70 95 23 04 60 11 42 69 24 68 56 01 32 56 71 37 02 36 91
	22 31 16 71 51 67 63 89 41 92 36 54 22 40 40 28 66 33 13 80
	24 47 32 60 99 03 45 02 44 75 33 53 78 36 84 20 35 17 12 50
	32 98 81 28 64 23 67 10 26 38 40 67 59 54 70 66 18 38 64 70
	67 26 20 68 02 62 12 20 95 63 94 39 63 08 40 91 66 49 94 21
	24 55 58 05 66 73 99 26 97 17 78 78 96 83 14 88 34 89 63 72
	21 36 23 09 75 00 76 44 20 45 35 14 00 61 33 97 34 31 33 95
	78 17 53 28 22 75 31 67 15 94 03 80 04 62 16 14 09 53 56 92
	16 39 05 42 96 35 31 47 55 58 88 24 00 17 54 24 36 29 85 57
	86 56 00 48 35 71 89 07 05 44 44 37 44 60 21 58 51 54 17 58
	19 80 81 68 05 94 47 69 28 73 92 13 86 52 17 77 04 89 55 40
	04 52 08 83 97 35 99 16 07 97 57 32 16 26 26 79 33 27 98 66
	88 36 68 87 57 62 20 72 03 46 33 67 46 55 12 32 63 93 53 69
	04 42 16 73 38 25 39 11 24 94 72 18 08 46 29 32 40 62 76 36
	20 69 36 41 72 30 23 88 34 62 99 69 82 67 59 85 74 04 36 16
	20 73 35 29 78 31 90 01 74 31 49 71 48 86 81 16 23 57 05 54
	01 70 54 71 83 51 54 69 16 92 33 48 61 43 52 01 89 19 67 48
`

const text2 = `
	01 02 03 04 05
	02 02 02 02 04
	03 03 03 03 03
	04 04 04 04 02
	05 05 05 05 99
`

/*
const text = `


	01 02 03 04 05 = 14
	02 02 02 02 04 = 10
	03 03 03 03 03 = 12
	04 04 04 04 02 = 16
	05 05 05 05 99 = 114
    -------------- \
    14 14 14 14 108 108
`
*/

const (
	fastMode = true
	gridSize = 5
)

var grid [][]int = [][]int{}

var rr = -1
var cc = -1
var direction = ""

func populateGrid() {
	fields := strings.Fields(text)
	index := 0
	for len(grid) < gridSize {
		row := []int{}
		for len(row) < gridSize {
			if fields[index] != "" {
				v, _ := strconv.Atoi(fields[index])
				row = append(row, v)
				fmt.Printf("%02d ", v)
			}
			index++
		}
		grid = append(grid, row)
		fmt.Println()
	}
}

func calculateHorizontalProduct(row, col int) int {
	result := 1
	times := 4
	if fastMode {
		if col > gridSize-4 {
			times = gridSize - col
		}
	}
	for i := 0; i < times; i++ {
		fmt.Println(row, col, i)
		v := grid[row][col+i]
		if fastMode {
			if v == 0 {
				return 0
			}
		}
		result *= v
	}
	return result
}

func calculateVerticalProduct(row, col int) int {
	result := 1
	times := 4
	if fastMode {
		if row > gridSize-4 {
			times = gridSize - row
		}
	}
	for i := 0; i < times; i++ {
		v := grid[row+i][col]

		if fastMode {
			if v == 0 {
				return 0
			}
		}
		result *= v
	}
	return result
}

func calculateDiagonalProduct(row, col int) int {
	result := 1

	times := 4
	if fastMode {
		if row > gridSize-4 || col > gridSize-4 {
			if row > col {
				times = gridSize - row
			} else {
				times = gridSize - col
			}
		}
	}

	for i := 0; i < times; i++ {
		v := grid[row+i][col+i]
		if fastMode {
			if v == 0 {
				return 0
			}
		}
		result *= v
	}
	return result
}

func calculateHorizontalSum(row, col int) int {
	result := 0
	times := 4
	if fastMode {
		if col > gridSize-4 {
			times = gridSize - col
		}
	}
	for i := 0; i < times; i++ {
		v := grid[row][col+i]
		if fastMode {
			if v == 0 {
				return 0
			}
		}
		result += v
	}
	return result
}

func calculateVerticalSum(row, col int) int {
	result := 0
	times := 4
	if fastMode {
		if row > gridSize-4 {
			times = gridSize - row
		}
	}
	for i := 0; i < times; i++ {
		v := grid[row+i][col]

		if fastMode {
			if v == 0 {
				return 0
			}
		}
		result += v
	}
	return result
}

func calculateDiagonalSum(row, col int) int {
	result := 0

	times := 4
	if fastMode {
		if row > gridSize-4 || col > gridSize-4 {
			if row > col {
				times = gridSize - row
			} else {
				times = gridSize - col
			}
		}
	}

	for i := 0; i < times; i++ {
		v := grid[row+i][col+i]
		if fastMode {
			if v == 0 {
				return 0
			}
		}
		result += v
	}
	return result
}

func calculateReverseDiagonalSum(row, col int) int {
	result := 0

	times := 4
	if fastMode {
		if row < 4 || col > gridSize-4 {
			if gridSize-1-row > col {
				times = row + 1
			} else {
				times = gridSize - col
			}
		}
	}

	for i := 0; i < times; i++ {
		v := grid[row-i][col+i]
		if fastMode {
			if v == 0 {
				return 0
			}
		}
		result += v
	}
	return result
}

func findLargestProductInRow(row int) int {
	limit := gridSize
	if fastMode {
		limit = gridSize - 3
	}

	result := 0
	for i := 0; i < limit; i++ {
		p := calculateHorizontalProduct(row, i)
		if p > result {
			result = p
		}
	}
	return result
}

func findLargestProductInCol(col int) int {
	limit := gridSize
	if fastMode {
		limit = gridSize - 3
	}

	result := 0
	for i := 0; i < limit; i++ {
		p := calculateVerticalProduct(i, col)
		if p > result {
			result = p
		}
	}
	return result
}

func findLargestProductInDiagonal(row, col int) int {
	dim := row
	if col > row {
		dim = col
	}

	limit := gridSize + 1 - dim
	if fastMode {
		if dim > gridSize-3 {
			return 0
		}
	}
	result := 0
	for i := 0; i < limit; i++ {
		p := calculateDiagonalProduct(row+i, col+i)
		if p > result {
			result = p
		}
	}
	return result
}

func findLargestSumInRow(row int) (result int, y int) {
	limit := gridSize
	if fastMode {
		limit = gridSize - 3
	}

	for i := 0; i < limit; i++ {
		p := calculateHorizontalSum(row, i)
		if p > result {
			result = p
			y = i
		}
	}
	return
}

func findLargestSumInCol(col int) (result int, x int) {
	limit := gridSize
	if fastMode {
		limit = gridSize - 3
	}

	for i := 0; i < limit; i++ {
		p := calculateVerticalSum(i, col)
		if p > result {
			result = p
			x = i
		}
	}
	return
}

func findLargestSumInDiagonal(row, col int) (result int, x int, y int) {
	dim := row
	if col > row {
		dim = col
	}

	limit := gridSize + 1 - dim
	if fastMode {
		if dim > gridSize-3 {
			return 0, -1, -1
		}
	}

	for i := 0; i < limit; i++ {
		p := calculateDiagonalSum(row+i, col+i)
		if p > result {
			result = p
			x = row + i
			y = col + i
		}
	}
	return
}

func findLargestSumInReverseDiagonal(row, col int) (result int, x int, y int) {
	dim := gridSize - 1 - row
	if col > row {
		dim = col
	}

	limit := gridSize + 1 - dim
	if fastMode {
		if dim > gridSize-3 {
			return 0, -1, -1
		}
	}

	for i := 0; i < limit; i++ {
		p := calculateReverseDiagonalSum(row-i, col+i)
		if p > result {
			result = p
			x = row - i
			y = col + i
		}
	}
	return
}

func findLargestProductInGrid() int {
	largestProduct := 0
	for i := 0; i < gridSize; i++ {
		product := findLargestProductInRow(i)
		if product > largestProduct {
			largestProduct = product
		}
		product = findLargestProductInCol(i)
		if product > largestProduct {
			largestProduct = product
		}
		product = findLargestProductInDiagonal(i, 0)
		if product > largestProduct {
			largestProduct = product
		}
		if i > 0 {
			product = findLargestProductInDiagonal(0, i)
			if product > largestProduct {
				largestProduct = product
			}
		}
	}
	return largestProduct
}

func findLargestSumInGrid() int {
	largestSum := 0
	for i := 0; i < gridSize; i++ {
		sum, y := findLargestSumInRow(i)
		if sum > largestSum {
			largestSum = sum
			direction = "h"
			rr = i
			cc = y
		}
		sum, x := findLargestSumInCol(i)
		if sum > largestSum {
			largestSum = sum
			direction = "v"
			rr = x
			cc = i
		}
		sum, x, y = findLargestSumInDiagonal(i, 0)
		if sum > largestSum {
			largestSum = sum
			direction = "d"
			rr = i
			cc = 0
		}
		sum, x, y = findLargestSumInReverseDiagonal(i, 0)
		if sum > largestSum {
			largestSum = sum
			direction = "rd"
			rr = gridSize-1-i
		}

		if i > 0 {
			sum, x, y = findLargestSumInDiagonal(0, i)
			if sum > largestSum {
				largestSum = sum
				direction = "d"
				rr = x
				cc = y
			}

			sum, x, y = findLargestSumInReverseDiagonal(0, i)
			if sum > largestSum {
				largestSum = sum
				direction = "rd"
				rr = x
				cc = y
			}
		}
	}
	return largestSum
}

func calculateProduct() (product int) {
	product = 1
	if direction == "h" {
		for i := 0; i < 4; i++ {
			product *= grid[rr][cc+i]
		}
	} else if direction == "v" {
		for i := 0; i < 4; i++ {
			product *= grid[rr+i][cc]
		}
	} else if direction == "d" {
		for i := 0; i < 4; i++ {
			product *= grid[rr+i][cc]
		}
	} else {
		for i := 0; i < 4; i++ {
			product *= grid[rr-i][cc]
		}
	}
	return
}

func main() {
	populateGrid()
	//result := findLargestProductInGrid()
	//fmt.Printf("The largest product of 4 adjacent values is: %d\n", result)
	result := findLargestSumInGrid()
	fmt.Printf("The largest sum of 4 adjacent values is: %d\n", result)
	fmt.Printf("The largest product of 4 adjacent values is: %d\n", calculateProduct())
}
