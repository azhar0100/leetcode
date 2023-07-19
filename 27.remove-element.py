#
# @lc app=leetcode id=27 lang=python3
#
# [27] Remove Element
#
from typing import List

# @lc code=start


class Solution:
    def removeElement(self, nums: List[int], val: int) -> int:
        n = len(nums)
        i = 0
        while i < n:
            x  = nums[i]
            if x == val:
                del nums[i]
                n -= 1
            else:
                i += 1
        return len(nums)
            
# @lc code=end

