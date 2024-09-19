# Created by Luigi Menale at 2024/09/18 16:40
# leetgo: 1.4.9
# https://leetcode.com/problems/two-sum/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        # Hashmap (dictionary) to store values and their indices
        prevMap = {} # val : index

        # Loop through each number in the array
        for index, x in enumerate(nums) :
            # x + y = target -> diff{y} = target - x
            diff = target - x
            if diff in prevMap:
                return [prevMap[diff], index]
            prevMap[x] = index

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    target: int = deserialize("int", read_line())
    ans = Solution().twoSum(nums, target)
    print("\noutput:", serialize(ans, "integer[]"))
