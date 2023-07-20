#
# @lc app=leetcode id=35 lang=python3
#
# [35] Search Insert Position
#
import bisect
# @lc code=start

class Solution:
    def searchInsert(self, nums: List[int], target: int) -> int:
        return bisect.bisect_left(nums, target)
# @lc code=end
# sol = Solution()
# sol.searchInsert([1,3,5,6],5)

