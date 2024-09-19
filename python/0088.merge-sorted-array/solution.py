# Created by Luigi Menale at 2024/09/19 11:08
# leetgo: 1.4.9
# https://leetcode.com/problems/merge-sorted-array/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        # Last index nums1
        last = m + n - 1

        # merge in reverse order
        while m > 0 and n > 0:
            if nums1[m - 1] > nums2[n - 1]:
                nums1[last] = nums1[m - 1]
                m -= 1
            else:
                nums1[last] = nums2[n - 1]
                n -= 1
            last -= 1
        # fill nums1 with leftover nums2 elements
        while n > 0:
            nums1[last] = nums2[n - 1]
            n, last = n - 1, last -1

# @lc code=end

if __name__ == "__main__":
    nums1: List[int] = deserialize("List[int]", read_line())
    m: int = deserialize("int", read_line())
    nums2: List[int] = deserialize("List[int]", read_line())
    n: int = deserialize("int", read_line())
    merge(nums1, m, nums2, n)
    ans = nums1
    print("\noutput:", serialize(ans, "List[int]"))
