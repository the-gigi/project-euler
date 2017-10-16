package main

import (
	"testing"
)

func TestFindPrime10001(t *testing.T) {
	for i := 0; i < 10; i++ {
		p10001 := findPrime10001()
		expected := 104743
		if p10001 != expected {
			t.Errorf("Failed to produce correct answer")
		}
	}
}

