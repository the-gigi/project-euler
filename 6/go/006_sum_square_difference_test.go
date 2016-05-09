package main

import (
	"testing"
	"reflect"
)

type List2ListTestPair struct {
	input  []int
	output []int
}

type Int2ListTestPair struct {
	input  int
	output []int
}

type List2IntTestPair struct {
	input  []int
	output int
}

type Int2IntTestPair struct {
	input  int
	output int
}

func TestSquareList(t *testing.T) {
	var tests = []List2ListTestPair{
		{[]int{}, []int{}},
		{[]int{1}, []int{1}},
		{[]int{2}, []int{4}},
		{[]int{3, 5, 7}, []int{9, 25, 49}},
	}

	for _, pair := range tests {
		result := SquareList(pair.input)
		if !reflect.DeepEqual(result, pair.output) {
			t.Error(
				"For input: ", pair.input,
				"expected:", pair.output,
				"got:", result)
		}
	}
}

func TestMakeIntList(t *testing.T) {
	var tests = []Int2ListTestPair{
		{0, []int{}},
		{1, []int{1}},
		{2, []int{1, 2}},
		{5, []int{1, 2, 3, 4 ,5}},
	}

	for _, pair := range tests {
		result := MakeIntList(pair.input)
		if !reflect.DeepEqual(result, pair.output) {
			t.Error(
				"For input: ", pair.input,
				"expected:", pair.output,
				"got:", result)
		}
	}
}

func TestSumList(t *testing.T) {
	var tests = []List2IntTestPair{
		{[]int{}, 0},
		{[]int{1}, 1},
		{[]int{1, 2}, 3},
		{[]int{12, 13, 25, 7}, 57},
	}

	for _, pair := range tests {
		result := SumList(pair.input)
		if result != pair.output {
			t.Error(
				"For input: ", pair.input,
				"expected:", pair.output,
				"got:", result)
		}
	}
}

func TestProcess(t *testing.T) {
	var tests = []Int2IntTestPair {
		{0, 0},
		{1, 0},
		{2, 4},
		{100, 25164150},
	}

	for _, pair := range tests {
		result := Process(pair.input)
		if result != pair.output {
			t.Error(
				"For input: ", pair.input,
				"expected:", pair.output,
				"got:", result)
		}
	}
}

func TestSumList_NotIdiomatic(t *testing.T) {
	// Test []{} -> 0
	result := SumList([]int{})
	if result != 0 {
			t.Error(
				"For input: ", []int{},
				"expected:", 0,
				"got:", result)
	}

	// Test []{4, 8, 9} -> 21
	result = SumList([]int{4, 8, 9})
	if result != 21 {
			t.Error(
				"For input: ", []int{},
				"expected:", 0,
				"got:", result)
	}
}



