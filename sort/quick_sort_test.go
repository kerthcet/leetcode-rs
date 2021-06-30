package sort

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestQuickSortWithEmptySlice(t *testing.T) {
	nums := []int{}
	assert.Nil(t, quickSort(nums), "error with empty slice")
}

func TestFindPivot(t *testing.T) {
	assert.Equal(t, 2, findPivot(0, 5, []int{2, 1, 4, 5, 8, 3}), "error find pivot")
}

func TestFindPivotWithEmptySlice(t *testing.T) {
	assert.Equal(t, -1, findPivot(0, 5, []int{}), "error find pivot with empty slice")
}

func TestQuickSortWithFullSlice(t *testing.T) {
	nums := []int{1, 4, 6, 3, 7, 98, 53}
	assert.Equal(t, []int{1, 3, 4, 6, 7, 53, 98}, quickSort(nums), "error with quich sort")
}
