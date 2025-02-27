from collections import Counter, deque
from typing import List
class Solution:
    def isValid(self, s: str) -> bool:
        stack = deque()
        brackets = '(){}[]'
        match_left = dict(zip(brackets[::2],brackets[1::2]))
        match_right = dict(zip(brackets[1::2],brackets[::2]))
        # print(match_left, match_right)
        for x in s:
            if x in match_left:
                stack.append(x)
            if x in match_right:
                try:
                    stack_val = stack.pop()
                    if not match_left[stack_val] == x:
                        return False
                except IndexError:
                    return False
        return len(stack) == 0

    def generateParenthesis(self, n: int) -> List[str]:
        solutions = []
        stack = deque()
        stack.append(('',0,0))
        while len(stack) > 0:
            args = stack.pop()
            old, n_l, n_r = args
            if n_l < n:
                stack.append((old+'(',n_l+1,n_r))
                # continue
            if n_r < n and n_r < n_l:
                stack.append((old+')',n_l,n_r+1))
                # continue
            if n_l == n and n_r == n:
                solutions.append(old)
                # continue

        # def generate(old, n_l, n_r):                
        #     if n_l < n:
        #         generate(old+'(',n_l+1,n_r)
        #     if n_r < n and n_r < n_l: 
        #         generate(old+')',n_l,n_r+1)
        #     if n_l == n and n_r == n:
        #         # if self.isValid(old):
        #         solutions.append(old)
        # generate('',0,0)
        return solutions

sol=Solution()
print(sol.generateParenthesis(2))