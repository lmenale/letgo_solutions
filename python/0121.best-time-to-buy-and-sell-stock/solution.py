# Created by Luigi Menale at 2024/09/19 14:23
# leetgo: 1.4.9
# https://leetcode.com/problems/best-time-to-buy-and-sell-stock/

from typing import *
from leetgo_py import *

# @lc code=begin

class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        profit = 0
        buy = prices[0]
        
        for p in prices:
            buy = min(buy, p)
            profit = max(profit, p - buy)
        return profit

# @lc code=end

if __name__ == "__main__":
    prices: List[int] = deserialize("List[int]", read_line())
    ans = Solution().maxProfit(prices)
    print("\noutput:", serialize(ans, "integer"))
