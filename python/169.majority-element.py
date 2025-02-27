#
# @lc app=leetcode id=169 lang=python3
#
# [169] Majority Element
#
from collections import Counter


# @lc code=start
class Solution:
    def majorityElement(self, nums: List[int]) -> int:
        count = Counter(nums)
        for k,v in count.items():
            if v > len(nums)//2:
                return k
        
# @lc code=end

