#
# @lc app=leetcode id=141 lang=python3
#
# [141] Linked List Cycle
#
class ListNode:
    def __init__(self, x):
        self.val = x
        self.next = None

# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None

from typing import Optional

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        visited = []
        head_node = head
        while head.next is not None:
            if head_node not in visited:
                visited.append(head_node)
            else:
                return True
        return False

        
        
# @lc code=end

