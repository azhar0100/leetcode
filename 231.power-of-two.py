#
# @lc app=leetcode id=231 lang=python3
#
# [231] Power of Two
#

# @lc code=start
from collections import Counter


class Solution:
    def hammingWeight(self, n: int) -> int:
        return Counter(bin(n)[2:])['1']
    
    def isPowerOfTwo(self, n: int) -> bool:
        if n < 0:
            return False
        return self.hammingWeight(n) == 1
# @lc code=end

