# Created by Luigi Menale at 2024/09/19 15:18
# leetgo: 1.4.9
# https://leetcode.com/problems/3sum/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def threeSum(self, nums: List[int]) -> List[List[int]]:
        res = []
        nums.sort()

        for i, a in enumerate(nums):
            if i > 0 and a == nums[i - 1]:
                continue

            left_index, right_index = i + 1, len(nums) - 1
            while left_index < right_index:
                threeSum = a + nums[left_index] + nums[right_index]
                if threeSum > 0:
                    right_index -= 1
                elif threeSum < 0:
                    left_index += 1
                else:
                    res.append([a, nums[left_index], nums[right_index]])
                    left_index += 1
                    while nums[left_index] == nums[left_index - 1] and left_index < right_index:
                        left_index += 1
        return res

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    ans = Solution().threeSum(nums)
    print("\noutput:", serialize(ans, "integer[][]"))
