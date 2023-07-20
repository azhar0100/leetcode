#
# @lc app=leetcode id=70 lang=python3
#
# [70] Climbing Stairs
#

# @lc code=start
from functools import lru_cache
class Solution:
    def climbStairs(self, n: int) -> int:
        @lru_cache
        def climb(n):
            if n == 1:
                return 1
            elif n == 2:
                return 2
            elif n == 3:
                return 3
            else:
                total = 0
                total += climb(n-1)
                total += climb(n-2)
                return total
        return climb(n)
        
# @lc code=end
# sol = Solution()
# print(sol.climbStairs(4))
