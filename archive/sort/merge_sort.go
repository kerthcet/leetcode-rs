package sort

func mergeSort(nums []int) []int {
	return merge(0, len(nums)-1, nums)
}

func merge(l, r int, nums []int) []int {
	if l > r {
		return nil
	}

	if l == r {
		return []int{nums[l]}
	}

	m := (r-l)/2 + l
	return mergeTwoOrderedList(merge(l, m, nums), merge(m+1, r, nums))
}

func mergeTwoOrderedList(nums1, nums2 []int) (res []int) {
	if len(nums1) == 0 && len(nums2) == 0 {
		return nil
	}

	var l, r int

	for {
		if l == len(nums1) {
			res = append(res, nums2[r:]...)
			return res
		}

		if r == len(nums2) {
			res = append(res, nums1[l:]...)
			return res
		}

		if nums1[l] > nums2[r] {
			res = append(res, nums2[r])
			r++
		} else {
			res = append(res, nums1[l])
			l++
		}
	}
}
