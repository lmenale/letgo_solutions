# Created by Luigi Menale at 2024/09/19 09:59
# leetgo: 1.4.9
# https://leetcode.com/problems/remove-duplicates-from-sorted-array/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        # Indicate the unique value
        left_index = 1

        # Loop through each number in the array
        for right_index in range(1, len(nums)):
            # Check if it's a unique value
            if nums[right_index] != nums[right_index - 1]:
                nums[left_index] = nums[right_index]
                left_index += 1
        return left_index

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    ans = Solution().removeDuplicates(nums)
    print("\noutput:", serialize(ans, "integer"))
