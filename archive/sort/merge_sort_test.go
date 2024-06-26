package sort

import (
	"testing"

	"github.com/stretchr/testify/assert"
)

func TestMergeSortWithEmptySlice(t *testing.T) {
	nums := []int{}
	assert.Nil(t, mergeSort(nums), "error with empty slice")
}

func TestMergeSortWithFullSlice(t *testing.T) {
	nums := []int{2, 4, 65, 1, 5, 23, 9}
	assert.Equal(t, []int{1, 2, 4, 5, 9, 23, 65}, mergeSort(nums), "error with full slice")
}

func TestMergeTwoOrderedList(t *testing.T) {
	l := []int{1, 3, 4}
	r := []int{2, 5, 9}

	assert.Equal(t, []int{1, 2, 3, 4, 5, 9}, mergeTwoOrderedList(l, r), "wrong merged list")
}

func TestMergeTwoOrderedListWithOneEmptySlice(t *testing.T) {
	l := []int{}
	r := []int{1, 2, 4}

	assert.Equal(t, []int{1, 2, 4}, mergeTwoOrderedList(l, r), "wrong merged list")
}

func TestMergeTwoOrderedListWithTwoEmptySlice(t *testing.T) {
	l := []int{}
	r := []int{}

	assert.Nil(t, mergeTwoOrderedList(l, r), "wrong merged list")
}
