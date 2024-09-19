# Created by Luigi Menale at 2024/09/19 11:19
# leetgo: 1.4.9
# https://leetcode.com/problems/convert-sorted-array-to-binary-search-tree/

from typing import *
from leetgo_py import *

# @lc code=begin

# Definition for a binary tree node.
# class TreeNode:
#     def __init__(self, val=0, left=None, right=None):
#         self.val = val
#         self.left = left
#         self.right = right
class Solution:
    def sortedArrayToBST(self, nums: List[int]) -> Optional[TreeNode]:
        # Nested helper function
        def helper(left, right):
            if left > right:
                return None
            
            m = (left + right) // 2
            root = TreeNode(nums[m])
            root.left = helper(left, m - 1)
            root.right = helper(m + 1, right)
            return root
        
        return helper(0, len(nums) - 1)

# @lc code=end

if __name__ == "__main__":
    nums: List[int] = deserialize("List[int]", read_line())
    ans = Solution().sortedArrayToBST(nums)
    print("\noutput:", serialize(ans, "TreeNode"))
