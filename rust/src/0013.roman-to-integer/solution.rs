// Created by Luigi Menale at 2024/10/08 05:45
// leetgo: 1.4.9
// https://leetcode.com/problems/roman-to-integer/

use anyhow::Result;
use leetgo_rs::*;

struct Solution;

// @lc code=begin

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let mut total = 0;
        let mut prev_value = 0;

        for c in s.chars().rev() {
            let value = match c {
                'I' => 1,
                'V' => 5,
                'X' => 10,
                'L' => 50,
                'C' => 100,
                'D' => 500,
                'M' => 1000,
                _ => 0,
            };

            if value < prev_value {
                total -= value;
            } else {
                total += value;
            }

            prev_value = value;
        }

        total
    }
}

// @lc code=end

fn main() -> Result<()> {
    let s: String = deserialize(&read_line()?)?;
    let ans: i32 = Solution::roman_to_int(s).into();

    println!("\noutput: {}", serialize(ans)?);
    Ok(())
}
