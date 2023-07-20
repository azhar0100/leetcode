#
# @lc app=leetcode id=83 lang=python3
#
# [83] Remove Duplicates from Sorted List
#
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
    
    def __repr__(self) -> str:
        return repr(self.val)
from typing import Optional

# @lc code=start
# Definition for singly-linked list.


class Solution:

    def list_refs(self, linked_list):
        output_l = []
        p_node = linked_list
        while not p_node is None:
            output_l.append(p_node)
            p_node = p_node.next
        return output_l

    def list_refs_to_node_list(self, refs):
        node_start = ListNode()
        p = node_start
        for x in refs:
            p.next = x
            p = p.next
        p.next = None
        return node_start.next
    
    def deleteDuplicates(self, head: Optional[ListNode]) -> Optional[ListNode]:
        p = head
        if p is None:
            return head
        n = 0
        while not p.next is None:
            x, y = p.val,p.next.val
            if x == y:
                p.next = p.next.next
            else:
                p = p.next
                n += 1
        return head


# @lc code=end

sol = Solution()
sol.deleteDuplicates(ListNode(1,ListNode(1,ListNode(2))))
print(sol.list_refs(sol.deleteDuplicates(ListNode(1,ListNode(1,ListNode(2))))))