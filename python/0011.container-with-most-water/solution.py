# Created by Luigi Menale at 2024/09/19 14:54
# leetgo: 1.4.9
# https://leetcode.com/problems/container-with-most-water/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def maxArea(self, height: List[int]) -> int:
        result = 0
        left_index, right_index = 0, len(height) - 1

        while left_index < right_index:
            area = (right_index - left_index) * min(height[left_index], height[right_index])
            result = max(result, area)
            if height[left_index] < height[right_index]:
                left_index += 1
            else:
                right_index -= 1

        return result

# @lc code=end

if __name__ == "__main__":
    height: List[int] = deserialize("List[int]", read_line())
    ans = Solution().maxArea(height)
    print("\noutput:", serialize(ans, "integer"))
