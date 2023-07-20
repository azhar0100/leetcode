#
# @lc app=leetcode id=136 lang=python3
#
# [136] Single Number
#
from collections import Counter
# @lc code=start
class Solution:
    def singleNumber(self, nums: List[int]) -> int:
        return {v:k for k,v in Counter(nums).items()}[1]
        
# @lc code=end

