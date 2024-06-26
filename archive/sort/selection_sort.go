package sort

func selectonSort(nums []int) []int {
	if len(nums) == 0 {
		return nil
	}

	for i := 0; i < len(nums)-1; i++ {
		position := i
		for j := i + 1; j < len(nums); j++ {
			if nums[position] > nums[j] {
				position = j
			}
		}
		nums[i], nums[position] = nums[position], nums[i]
	}

	return nums
}
