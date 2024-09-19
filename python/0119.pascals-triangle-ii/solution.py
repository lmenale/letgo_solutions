# Created by Luigi Menale at 2024/09/19 14:18
# leetgo: 1.4.9
# https://leetcode.com/problems/pascals-triangle-ii/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def getRow(self, rowIndex: int) -> List[int]:
        res = [1]

        for i in range(rowIndex):
            next_row = [0] * (len(res) + 1)
            for j in range(len (res)):
                next_row[j] += res[j]
                next_row[j + 1] += res[j]
            res = next_row
        return res

# @lc code=end

if __name__ == "__main__":
    rowIndex: int = deserialize("int", read_line())
    ans = Solution().getRow(rowIndex)
    print("\noutput:", serialize(ans, "integer[]"))
