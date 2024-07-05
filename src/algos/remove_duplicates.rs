// Description: https://leetcode.cn/problems/remove-duplicates-from-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0
        }

        let mut m: usize = 0;
        for i in 1..nums.len() {
            if nums[i] == nums[m] {
                continue
            }

            nums[m+1] = nums[i];
            m += 1;
        }
        return m as i32 +1
    }
}

fn main() {
    let mut nums1: Vec<i32> = vec![];
    let value1 = Solution::remove_duplicates(&mut nums1);
    assert_eq!(0, value1, "value1 is not right");

    let mut nums2: Vec<i32> = vec![0,0,1,1,1,2,2,3,3,4];
    let value2 = Solution::remove_duplicates(&mut nums2);
    assert_eq!(5, value2, "value2 is not right");
    assert_eq!(vec![0,1,2,3,4], nums2[..5], "nums2 is not right");
}