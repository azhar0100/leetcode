#
# @lc app=leetcode id=1275 lang=python3
#
# [1275] Find Winner on a Tic Tac Toe Game
#

from typing import List
# @lc code=start
import numpy as np
class Solution:
    def tictactoe(self, moves: List[List[int]]) -> str:
        board = np.zeros((3,3),dtype=int)
        for i,x in enumerate(moves):
            board[x[0],x[1]] = 1 if i % 2 == 0 else -1
        
        players = {
            1: 'A',
            -1: 'B'
        }
        
        winning_conditions = []
        winning_conditions.extend([[[i,j] for j in range(3)] for i in range(3)])
        winning_conditions.extend([[[j,i] for j in range(3)] for i in range(3)])
        winning_conditions.append([[0,0],[1,1],[2,2]])
        winning_conditions.append([[2,0],[1,1],[0,2]])
        for win_cond in winning_conditions:
            board_line = [board[x,y] for x, y in win_cond]
            board_cont = list(set(board_line))
            if len(board_cont) == 1:
                if board_cont[0] != 0:
                    print(f'{players[board_cont[0]]} won')
                    return players[board_cont[0]]
        else:
            if (board == 0).sum() > 0:
                return 'Pending'
            else:
                return 'Draw'
        # return board
        
# @lc code=end
print(Solution().tictactoe([[0,0],[2,0],[1,1],[2,1],[2,2]]))

