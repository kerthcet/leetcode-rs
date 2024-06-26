package sort

func quickSort(nums []int) []int {
	return partiton(0, len(nums)-1, nums)
}

func partiton(l, r int, nums []int) []int {
	if l > r {
		return nil
	}

	if l == r {
		return []int{nums[l]}
	}

	pivot := findPivot(l, r, nums)
	nums1 := partiton(l, pivot-1, nums)
	nums2 := partiton(pivot+1, r, nums)

	nums1 = append(nums1, nums[pivot])
	return append(nums1, nums2...)
}

func findPivot(l, r int, nums []int) int {
	if len(nums) == 0 {
		return -1
	}

	pivot := r
	for i := l; i < r; i++ {
		if nums[pivot] < nums[i] && pivot > i {
			nums[pivot], nums[i] = nums[i], nums[pivot]
			pivot = i
		}
	}

	return pivot
}
