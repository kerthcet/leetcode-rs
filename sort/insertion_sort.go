package sort

func insertionSort(nums []int) []int {
	if len(nums) == 0 {
		return nil
	}

	for i := 1; i < len(nums); i++ {
		val := nums[i]
		for j := i - 1; j >= 0; j-- {
			if val < nums[j] {
				nums[j+1] = nums[j]

				if j == 0 {
					nums[j] = val
				}
				continue
			}

			nums[j+1] = val
			break
		}
	}
	return nums
}
