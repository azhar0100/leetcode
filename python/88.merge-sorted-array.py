#
# @lc app=leetcode id=88 lang=python3
#
# [88] Merge Sorted Array
#
from typing import List, Optional

# @lc code=start


class Solution:
            
        
    def merge(self, nums1: List[int], m: int, nums2: List[int], n: int) -> None:
        """
        Do not return anything, modify nums1 in-place instead.
        """
        l3 = []
        n1, n2 = (m,n)
        i = 0
        j = 0
        while i < n1 and j < n2:
            if nums1[i] < nums2[j]:
                i += 1
            else:
                nums1.insert(i, nums2[j])
                del nums1[-1]
                j += 1
                i += 1
                n1 += 1
        while j < n2:
            nums1.insert(i, nums2[j])
            j += 1
            i += 1
            del nums1[-1]
        # for i,x in enumerate(l3):
        #     nums1[i] = x
        
        
        
        
# @lc code=end
sol = Solution()
sol.merge([2,0],1,[1],1)
