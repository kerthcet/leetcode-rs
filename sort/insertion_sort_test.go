package sort

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestInsertionSortWithEmptySlice(t *testing.T) {
	nums := []int{}
	assert.Nil(t, insertionSort(nums), "error with empty slice")
}

func TestInserionSortWithFullSlice(t *testing.T) {
	nums := []int{89, 6, 4, 2, 677, 3, 7}
	assert.Equal(t, []int{2, 3, 4, 6, 7, 89, 677}, insertionSort(nums), "error with full slice")
}
