// Description: https://leetcode.cn/problems/remove-element/description/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut m:i8 = 0;
        // Watch out here we can't use usize because n could be negative.
        let mut n:i8 = nums.len() as i8 - 1;

        while m <= n {
            if nums[m as usize] == val {
                if nums[n as usize] == val {
                    n -= 1;
                    continue
                }else{
                    nums[m as usize] = nums[n as usize];
                    // Watch out here n should not be negative, so we use i8 instead.
                    n -= 1;
                }
            }
            m += 1;
        }
        return m as i32
    }
}

fn main() {
    let mut nums1: Vec<i32> = vec![0,1,2,2,3,0,4,2];
    let value1: i32 = Solution::remove_element(&mut nums1, 2);
    assert_eq!(5, value1, "value1 not right");
    assert_eq!(vec![0,1,4,0,3], nums1[0..5], "nums1 not right");

    let mut nums2: Vec<i32> = vec![];
    let value2: i32 = Solution::remove_element(&mut nums2, 0);
    assert_eq!(0, value2, "value2 not right");
    let vec2: Vec<i32> = vec![];
    assert_eq!(vec2, nums2[0..0], "nums2 not right");

    let mut nums3: Vec<i32> = vec![1];
    let value3: i32 = Solution::remove_element(&mut nums3, 1);
    assert_eq!(0, value3, "value3 not right");
    let vec3: Vec<i32> = vec![];
    assert_eq!(vec3, nums3[0..0], "nums3 not right");

    println!("Succeed!")
}