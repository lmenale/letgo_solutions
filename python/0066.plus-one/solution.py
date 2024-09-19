# Created by Luigi Menale at 2024/09/19 10:54
# leetgo: 1.4.9
# https://leetcode.com/problems/plus-one/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def plusOne(self, digits: List[int]) -> List[int]:
        # Invert the array for the special case of 9
        digits = digits[::-1]
        is_one, index = True, 0

        while is_one:
            if index < len(digits):
                if digits[index] == 9:
                    digits[index] = 0
                else:
                    digits[index] += 1
                    is_one = False
            else:
                digits.append(1)
                is_one = False

            index += 1
        return digits[:: -1]

# @lc code=end

if __name__ == "__main__":
    digits: List[int] = deserialize("List[int]", read_line())
    ans = Solution().plusOne(digits)
    print("\noutput:", serialize(ans, "integer[]"))
