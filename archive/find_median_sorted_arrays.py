class Solution:
    def findMedianSortedArrays(self, nums1: List[int],
                               nums2: List[int]) -> float:
        length_num1, length_num2 = len(nums1), len(nums2)
        length = length_num1 + length_num2
        index_1, index_2 = 0, 0
        temp, value = 0, 0

        skip1 = True if length_num1 == 0 else False
        skip2 = True if length_num2 == 0 else False

        if length % 2 == 0:
            m = length / 2 - 1
            n = m + 1
        else:
            m = n = length // 2

        while True:
            if index_1 + index_2 == m + 1:
                value += temp

            if index_1 + index_2 == n + 1:
                value += temp
                break

            if skip1:
                temp = nums2[index_2]
                index_2 += 1
            elif skip2:
                temp = nums1[index_1]
                index_1 += 1
            else:
                if nums1[index_1] > nums2[index_2]:
                    temp = nums2[index_2]
                    index_2 += 1
                else:
                    temp = nums1[index_1]
                    index_1 += 1

            if index_1 == length_num1:
                skip1 = True

            if index_2 == length_num2:
                skip2 = True

        return float(value / 2)
