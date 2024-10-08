// Created by Luigi Menale at 2024/09/18 16:37
// leetgo: 1.4.9
// https://leetcode.com/problems/two-sum/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut num_to_index = HashMap::new();

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&index) = num_to_index.get(&complement) {
                return vec![index as i32, i as i32];
            }
            num_to_index.insert(num, i);
        }

        vec![] // This line should never be reached due to the problem constraints.
    }
}

// @lc code=end

fn main() -> Result<()> {
    let nums: Vec<i32> = deserialize(&read_line()?)?;
    let target: i32 = deserialize(&read_line()?)?;
    let ans: Vec<i32> = Solution::two_sum(nums, target).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
