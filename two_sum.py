class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        dict = {}
        for i, e in enumerate(nums):
            if target - e in dict:
                return [dict[target - e], i]
            dict[e] = i
