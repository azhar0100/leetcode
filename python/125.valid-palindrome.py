#
# @lc app=leetcode id=125 lang=python3
#
# [125] Valid Palindrome
#
# @lc code=start
class Solution:
    def isPalindrome(self, s: str) -> bool:
        ss = ''.join([x for x in s.lower() if x.isalnum()])
        return ss == ss[::-1]
        
# @lc code=end

