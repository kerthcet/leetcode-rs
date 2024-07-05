// Description: https://leetcode.cn/problems/merge-sorted-array/description/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut end1  = m as usize;
        let mut end2 = n as usize;

        for i in (0..(m+n) as usize).rev() {
            if end2 == 0 {
                return
            }

            if end1 == 0 {
                nums1[i] = nums2[end2-1];
                end2 -= 1;
                continue
            }

            if nums1[end1-1] >= nums2[end2-1] {
                nums1[i] = nums1[end1-1];
                end1 -= 1;
            } else{
                nums1[i] = nums2[end2-1];
                end2 -= 1;
            }
        }
    }
}

fn main() {
    let mut nums1: Vec<i32> = vec![1,2,3,0,0,0];
    let mut nums2: Vec<i32> = vec![2,5,6];
    let m: i32 = 3;
    let n: i32 = 3;

    Solution::merge(&mut nums1, m, &mut nums2, n);
    assert_eq!(vec![1,2,2,3,5,6], nums1, "nums1 not right");
    assert_eq!(vec![2,5,6], nums2, "nums2 not right");
}
