#
# @lc app=leetcode id=26 lang=python3
#
# [26] Remove Duplicates from Sorted Array
#

# @lc code=start


class Solution:
    def removeDuplicates(self, nums: List[int]) -> int:
        n = len(nums)
        i = 0
        while i < n-1:
            x, y = nums[i],nums[i+1]
            if x == y:
                del nums[i+1]
                n -= 1
            else:
                i += 1
        return len(nums)
            
        

        
# @lc code=end

# sol = Solution()
# nums = [1,1,2]
# res = sol.removeDuplicates(nums)
# print(res)
# print(nums)


