from collections import deque
from typing import Optional

    # class TreeNode:
    #     def __init__(self, val=0, left=None, right=None):
    #         self.val = val
    #         self.left = left
    #         self.right = right

class Solution:
    def recoverFromPreorder(self, traversal: str) -> Optional[TreeNode]:
        stack = []
        i = 0
        n = len(traversal)
        while i < n:
            # depth
            d = 0
            while i<n and traversal[i] == '-':
                d += 1
                i += 1
            
            # number
            num = 0
            while i<n and traversal[i].isdigit():
                num = num * 10 + int(traversal[i])
                i += 1
            

            # place
            node = TreeNode(num)
            if len(stack) == d: # left
                if stack: # root case
                    stack[-1].left = node
            else: # find the right node
                while len(stack) > d:
                    stack.pop()
                stack[-1].right = node
            
            stack.append(node)
        
        return stack[0]

