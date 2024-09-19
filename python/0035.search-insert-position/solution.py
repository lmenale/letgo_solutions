# Created by Luigi Menale at 2024/09/19 10:38
# leetgo: 1.4.9
# https://leetcode.com/problems/search-insert-position/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        # Binary search - Log(n)
        left_index, right_index = 0, len(nums) - 1

        while left_index <= right_index:
            mid = (left_index + right_index) // 2

            if target == nums[mid]:
                return mid
            
            if target > nums[mid]:
                left_index = mid + 1
            else:
                right_index = mid - 1
            
        return left_index

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    target: int = deserialize("int", read_line())
    ans = Solution().searchInsert(nums, target)
    print("\noutput:", serialize(ans, "integer"))
