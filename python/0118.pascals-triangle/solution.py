# Created by Luigi Menale at 2024/09/19 11:39
# leetgo: 1.4.9
# https://leetcode.com/problems/pascals-triangle/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def generate(self, numRows: int) -> List[List[int]]:
        res = [[1]]
        for i in range(numRows - 1):
            temp = [0] + res[-1] + [0]
            row = []
            for j in range(len(res[-1]) + 1):
                row.append(temp[j] + temp[j + 1])
            res.append(row)
        return res

# @lc code=end

if __name__ == "__main__":
    numRows: int = deserialize("int", read_line())
    ans = Solution().generate(numRows)
    print("\noutput:", serialize(ans, "integer[][]"))
