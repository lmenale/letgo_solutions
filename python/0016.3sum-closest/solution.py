# Created by Luigi Menale at 2024/09/19 16:28
# leetgo: 1.4.9
# https://leetcode.com/problems/3sum-closest/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def threeSumClosest(self, nums: List[int], target: int) -> int:
        nums.sort()
        n = len(nums)
        closest_sum = float('inf')

        for i in range(n):
            if i > 0 and nums[i] == nums[i - 1]:
                continue

            lo, hi = i + 1, n - 1
            while lo < hi:
                cur_sum = nums[i] + nums[lo] + nums[hi]

                if abs(cur_sum - target) < abs(closest_sum - target):
                    closest_sum = cur_sum
                
                if cur_sum == target:
                    return cur_sum
                elif cur_sum < target:
                    lo += 1
                else:
                    hi -= 1
        
        return closest_sum

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    target: int = deserialize("int", read_line())
    ans = Solution().threeSumClosest(nums, target)
    print("\noutput:", serialize(ans, "integer"))
