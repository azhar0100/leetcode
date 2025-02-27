#
# @lc app=leetcode id=21 lang=python3
#
# [21] Merge Two Sorted Lists
#

# @lc code=start
# Definition for singly-linked list.
class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next
    
    def __repr__(self):
        return repr(self.val)

from typing import Optional


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

    def mergeTwoLists(self, list1: Optional[ListNode], list2: Optional[ListNode]) -> Optional[ListNode]:
        l1 = self.list_refs(list1)
        l2 = self.list_refs(list2)
        l3 = []
        n1, n2 = (len(l1),len(l2))
        n = max(n1, n2)
        i = 0
        j = 0
        while i < n1 and j < n2:
            if l1[i].val < l2[j].val:
                l3.append(l1[i])
                i += 1
            else:
                l3.append(l2[j])
                j += 1
        while i < n1:
            l3.append(l1[i])
            i += 1
        while j < n2:
            l3.append(l2[j])
            j += 1
        return self.list_refs_to_node_list(l3)
        
            
        
# @lc code=end
args = []
args.append(ListNode(1,ListNode(2,ListNode(4))))
args.append(ListNode(1,ListNode(3,ListNode(4))))
sol = Solution()
print(sol.list_refs(sol.mergeTwoLists(*args)))

