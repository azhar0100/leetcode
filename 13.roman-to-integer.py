#
# @lc app=leetcode id=13 lang=python3
#
# [13] Roman to Integer
#

# @lc code=start

import itertools as it
from collections import deque

class Solution:
    def romanToInt(self, s: str) -> int:
        
        vals = {"I":1,"V":5,"X":10,"L":50,"C":100,"D":500,"M":1000}
        total = 0
        def group_yielder(s):
            stack = deque()
            for g in it.groupby(s):
                k, l = g
                p = (vals[k],sum(1 for _ in l))
                stack.append(p)
                if len(stack) == 2:
                    g2 = stack.pop()
                    g1 = stack.pop()
                    yield g1, g2
                    stack.append(g2)
            yield stack.pop(), None
        
        for g1, g2 in group_yielder(s):
            val1, n1 = g1
            if not g2 is None:
                val2, n2 = g2
            else:
                val2 = val1
            cur_val = val1 * n1
            if val1 < val2:
                total -= cur_val
            else:
                total += cur_val
        return total
        
# @lc code=end

sol = Solution()
print(sol.romanToInt('XIV'))