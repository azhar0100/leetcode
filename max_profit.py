from typing import List


class Solution:
    def maxProfit(self, prices: List[int]) -> int:
        def relu(x):
            if x > 0:
                return x
            else:
                return 0
        en_prices = [*enumerate(prices)]
        min_price_point = min(en_prices, key=lambda x: x[1])
        try:
            min_max_price_point = max(en_prices[min_price_point[0]+1:], key=lambda x: x[1])
        except ValueError as e:
            print(e)
            min_max_price_point = min_price_point
        max_price_point = max(en_prices, key=lambda x: x[1])
        try:
            max_min_price_point = min(en_prices[:max_price_point[0]], key=lambda x: x[1])
        except ValueError:
            max_min_price_point = max_price_point
        # print(min_price_point, min_max_price_point)

        pair1 = (min_price_point,min_max_price_point)
        pair2 = (max_min_price_point,max_price_point)
        pairs = [pair1, pair2]
        print(pairs)
        # valid_pairs = [(x,y) for x, y in pairs if y[1] > x[1]]
        # if len(valid_pairs) == 0:
        #     return 0
        # else:
        max_pair = max(pairs, key=lambda x: relu(x[1][1] - x[0][1]))
        x = max_pair
        return relu(x[1][1] - x[0][1])

sol = Solution()
print(sol.maxProfit([7,6,4,3,1]))