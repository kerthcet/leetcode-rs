// Description: https://leetcode.cn/problems/remove-duplicates-from-sorted-array-ii/description/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return 0
        }

        let mut m = 0;
        let mut count = 1;
        for i in 1..nums.len() {
            if nums[i] == nums[m] {
                if count < 2 {
                    nums[m+1] = nums[i];
                    m += 1;
                }
                count += 1;
            } else {
                nums[m+1] = nums[i];
                m += 1;
                count = 1;
            }
        }
        return m as i32 + 1
    }
}

fn main() {
    let mut nums1: Vec<i32> = vec![];
    let value1 = Solution::remove_duplicates(&mut nums1);
    assert_eq!(0, value1, "value1 is not right");

    let mut nums2: Vec<i32> = vec![1,1,1,2,2,3];
    let value2 = Solution::remove_duplicates(&mut nums2);
    assert_eq!(vec![1,1,2,2,3], nums2[..5], "nums2 is not right");
    assert_eq!(5, value2, "value2 is not right");

    println!("Succeed!");
}