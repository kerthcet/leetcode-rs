// Description: https://leetcode.cn/problems/3sum/?envType=study-plan-v2&envId=top-100-liked

struct Solution;

impl Solution {
    pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut res: Vec<Vec<i32>> = Vec::new();
        nums.sort();

        for i in 0..nums.len() - 2 {
            if nums[i] > 0 {
                break
            }
            if i > 0 && nums[i] == nums[i-1] {
                continue
            }
            for j in i+1..nums.len() - 1 {
                if nums[i] + nums[j] > 0 {
                    break
                }
                if j > i + 1 && nums[j] == nums[j-1] {
                    continue
                }
                for k in j+1..nums.len() {
                    if nums[i] + nums[j] + nums[k] == 0 {
                        res.push(vec![nums[i], nums[j], nums[k]]);
                        break
                    }
                    if nums[i] + nums[j] + nums[k] > 0 {
                        break
                    }
                }
            }
        }
        return res
    }
}

fn main() {
    let nums: Vec<i32> = vec![-1,0,1,2,-1,-4];
    let value: Vec<Vec<i32>> = vec![vec![-1,-1,2],vec![-1,0,1]];
    assert_eq!(value, Solution::three_sum(nums), "value not right");
}