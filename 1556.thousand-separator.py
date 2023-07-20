#
# @lc app=leetcode id=1556 lang=python3
#
# [1556] Thousand Separator
#

# @lc code=start
class Solution:
    def thousandSeparator(self, n: int) -> str:
        s = str(n)[::-1]
        s2 = []
        for i in range(0,len(s),3):
            s2.append(s[i:i+3])
        s3 = '.'.join(s2)
        return s3[::-1]
        
# @lc code=end
print(Solution().thousandSeparator(1234))
