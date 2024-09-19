# Created by Luigi Menale at 2024/09/19 14:27
# leetgo: 1.4.9
# https://leetcode.com/problems/single-number/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        result = 0

        for n in nums:
            result= result ^ n
        
        return result

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    ans = Solution().singleNumber(nums)
    print("\noutput:", serialize(ans, "integer"))
