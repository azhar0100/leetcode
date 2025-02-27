#
# @lc app=leetcode id=500 lang=python3
#
# [500] Keyboard Row
#
from typing import List

# @lc code=start


class Solution:
    def findWords(self, words: List[str]) -> List[str]:
        rows = [set(x) for x in ["qwertyuiop","asdfghjkl","zxcvbnm"]]
        nwords = []
        for word in words:
            intersections = sum([1 if len(set(word.lower()) & row) > 0 else 0 for row in rows])
            if intersections == 1:
                nwords.append(word)
        return nwords
        
# @lc code=end
print(Solution().findWords(["Hello","Alaska","Dad","Peace"]))
