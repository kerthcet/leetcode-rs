// Description: https://leetcode.cn/problems/rotate-array/description/?envType=study-plan-v2&envId=top-interview-150

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let l = nums.len();
        let offset = k as usize % l;
        nums.splice(0..0, vec![0; offset]);

        for i in l..nums.len() {
            nums[i-l] = nums[i];
        }
        nums.truncate(l)
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![1,2,3,4,5,6,7];
    Solution::rotate(&mut nums, 3);
    assert_eq!(vec![5,6,7,1,2,3,4], nums, "nums is not right");

    println!("Succeed!")
}