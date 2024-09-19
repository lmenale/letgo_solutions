# Created by Luigi Menale at 2024/09/19 10:21
# leetgo: 1.4.9
# https://leetcode.com/problems/remove-element/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        # Represent the index value
        unique_index = 0

        # Loop through each number in the array
        for index in range(len(nums)):
            # Check if it's a duplicate value
            if nums[index] != val:
                nums[unique_index] = nums[index]
                unique_index += 1
        return unique_index

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    val: int = deserialize("int", read_line())
    ans = Solution().removeElement(nums, val)
    print("\noutput:", serialize(ans, "integer"))
