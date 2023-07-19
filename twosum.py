from typing import List


class Solution:

    def binary_search(self, nums, val, af = lambda x:x):
        mid = len(nums) // 2
        if len(nums) == 0:
            return None

        if val > af(nums[mid]):
            return self.binary_search(nums[mid:], val, af)
        elif val < af(nums[mid]):
            return self.binary_search(nums[:mid], val, af)
        else:
            return mid, nums[mid]
        return None

    def twoSum(self, nums: List[int], target: int) -> List[int]:
        hashmap = {target-x:(i,x) for i,x in enumerate(nums)}
        for i,x in enumerate(nums):
            if x in hashmap:
                if i!=hashmap[x][0]:
                    return [i,hashmap[x][0]]
        
