#
# @lc app=leetcode id=14 lang=python3
#
# [14] Longest Common Prefix
#

# @lc code=start
from collections import deque
from typing import List


class Solution:
    def longestCommonPrefix(self, strs: List[str]) -> str:
        prefix = ""
        minlen = min([len(string) for string in strs])
        for i in range(minlen):
            strsubs = [st[i] for st in strs]
            if len(set(strsubs)) == 1:
                prefix += strs[0][i]
            else:
                break
        return prefix

            
        
        
# @lc code=end

