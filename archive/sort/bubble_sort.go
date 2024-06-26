package sort

func bubbleSort(nums []int) []int {
	if len(nums) == 0 {
		return nil
	}

	for i := range nums {
		for j := i + 1; j < len(nums); j++ {
			if nums[i] > nums[j] {
				nums[i], nums[j] = nums[j], nums[i]
			}
		}
	}

	return nums
}
