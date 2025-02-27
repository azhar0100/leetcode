#
# @lc app=leetcode id=191 lang=python3
#
# [191] Number of 1 Bits
#
from collections import Counter

# @lc code=start
class Solution:
    def hammingWeight(self, n: int) -> int:
        return Counter(bin(n)[2:])['1']
# @lc code=end

