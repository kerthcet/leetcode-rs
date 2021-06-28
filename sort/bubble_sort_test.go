package sort

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestBubbleSortWithEmptySlice(t *testing.T) {
	nums := []int{}
	assert.Nil(t, bubbleSort(nums), "error with empty slice")
}

func TestBubbleSortWithFullSlice(t *testing.T) {
	nums := []int{1, 23, 5, 6, 32, 2, 7, 98}
	actual := []int{1, 2, 5, 6, 7, 23, 32, 98}
	assert.Equal(t, actual, bubbleSort(nums), "error with full slice")
}
