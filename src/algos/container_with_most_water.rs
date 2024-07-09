// Description: https://leetcode.cn/problems/container-with-most-water/?envType=study-plan-v2&envId=top-100-liked

struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left: usize = 0;
        let mut right: usize = height.len() - 1;
        let mut h: i32 = 0;
        let mut left_h: i32 = 0;
        let mut right_h: i32 = 0;

        while left < right {
            if height[left] <= left_h {
                left += 1;
                continue
            } else if height[right] <= right_h {
                right -= 1;
                continue
            }
            let ac: i32 = (right - left) as i32 * height[left].min(height[right]);
            h = h.max(ac);
            if height[left] < height[right] {
                left_h = height[left];
                left += 1;
            }else{
                right_h = height[right];
                right -= 1;
            }
        }
        return h;
    }
}

fn main() {
    let height: Vec<i32> = vec![1,8,6,2,5,4,8,3,7];
    let value: i32 = Solution::max_area(height);
    assert_eq!(value, 49, "value not right");
}