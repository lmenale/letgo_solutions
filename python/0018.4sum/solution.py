# Created by Luigi Menale at 2024/09/19 17:44
# leetgo: 1.4.9
# https://leetcode.com/problems/4sum/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def fourSum(self, nums: List[int], target: int) -> List[List[int]]:
        nums.sort()
        res, quad = [], []

        def kSum(k, start, target):
            if k != 2:
                for i in range(start, len(nums) - k + 1):
                    if i > start and nums[i] == nums[i - 1]:
                        continue
                    quad.append(nums[i])
                    kSum(k - 1, i + 1, target - nums[i])
                    quad.pop()
                return
            # base case, two sum II
            left_index, right_index = start, len(nums) - 1
            while left_index < right_index:
                if nums[left_index] + nums[right_index] < target:
                    left_index += 1
                elif nums[left_index] + nums[right_index] > target:
                    right_index -= 1
                else:
                    res.append(quad + [nums[left_index], nums[right_index]])
                    left_index += 1
                    while left_index < right_index and nums[left_index] == nums[left_index - 1]:
                        left_index += 1
        
        kSum(4, 0, target)
        return res

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    target: int = deserialize("int", read_line())
    ans = Solution().fourSum(nums, target)
    print("\noutput:", serialize(ans, "integer[][]"))
