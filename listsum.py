class ListNode:
    def __init__(self, val=0, next=None):
        self.val = val
        self.next = next

import itertools
from typing import Optional
class Solution:
    def addTwoNumbers(self, l1: Optional[ListNode], l2: Optional[ListNode]) -> Optional[ListNode]:
        def expand_linked(l):
            p = l
            a = []
            while not p is None:
                yield p.val
                p = p.next
        def f(x):
            if x is None:
                return 0
            return x
        sumvalint = (sum([10**i * (f(x)+f(y)) for (i,(x,y)) in enumerate(itertools.zip_longest(expand_linked(l1),expand_linked(l2)))]))
        sumval = str(sumvalint)
        emptynode = ListNode()
        curnode = emptynode
        for x in (sumval[::-1]):
            curnode.next = ListNode(val=int(x))
            curnode = curnode.next
        return emptynode.next

def convert_sumval(sumval):
    sumval = str(sumval)
    emptynode = ListNode()
    curnode = emptynode
    for x in (sumval[::-1]):
        curnode.next = ListNode(val=int(x))
        curnode = curnode.next
    return emptynode.next

l1 = convert_sumval(9999999)
l2 = convert_sumval(9999)
def expand_linked(l):
    p = l
    a = []
    while not p is None:
        yield p.val
        p = p.next
print([*expand_linked(Solution.addTwoNumbers(None, l1, l2))])
