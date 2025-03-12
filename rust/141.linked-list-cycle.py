#
# @lc app=leetcode id=141 lang=python3
#
# [141] Linked List Cycle
#

# @lc code=start
# Definition for singly-linked list.
# class ListNode:
#     def __init__(self, x):
#         self.val = x
#         self.next = None


from typing import Optional

class Solution:
    def hasCycle(self, head: Optional[ListNode]) -> bool:
        visited = [head]
        head_node = getattr(head,'next',None)
        while head_node is not None:
            if head_node not in visited:
                visited.append(head_node)
                head_node = head_node.next
            else:
                return True
        return False


# @lc code=end

