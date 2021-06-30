package sort

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestSelectonSortWithEmptySlice(t *testing.T) {
	nums := []int{}
	assert.Nil(t, selectonSort(nums), "error with empty slice")
}

func TestSelectonSortWithFullSlice(t *testing.T) {
	nums := []int{5, 3, 1, 89, 53, 2}
	assert.Equal(t, []int{1, 2, 3, 5, 53, 89}, selectonSort(nums), "error with full slice")
}
