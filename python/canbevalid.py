import itertools as it

class Solution:

    def canBeValidBase(self, s: str, locked: str) -> bool:
        if len(s) % 2 == 1:
            return False
        # return True
        K = [0]
        S = []
        m = [0]
        slr = [1 if x == '(' else -1 for x in s]
        unlocked_amount = 0
        for i, x, l in zip(it.count(), slr, locked):
            # if i == 0:
            #     K.append(0)
            #     m.append(0)
            # else:
            if l == '1':
                K.append(K[-1] + x)
                m.append(m[-1])
            elif l == '0':
                unlocked_amount += 1
                K.append(K[-1])
                m.append(m[-1] + 1)
        K = K[1:]
        m = m[1:]
        S = [(-x,+x) for x in m]
        print(K)
        print(m)
        print(S)

        for i,k,s in zip(it.count(),K,S):
            # S[i] >= -K[i]
            mins, maxs = s
            if maxs < -K[i]:
                return False
            else:
                mins = max(-K[i], mins)
            s = mins, maxs
            S[i] = s

            # for js in S[:i:][::-1]:
            #     jmin, jmax = js


        print(S)
        final_range = S[-1]
        check_value = -K[-1]
        if check_value >= final_range[0] and check_value <= final_range[1]:
            return (unlocked_amount % 2) == (check_value % 2)
    
    def matchRightLocked(self, s, original_locked):
        original_s = s
        locked = original_locked
        locked_right = [i for i,x,l in zip(it.count(),s,locked) if l == '1' and x == ')']
        for count, locked_i in enumerate(locked_right,start=1):
            substr_locked_i = s[:locked_i]
            substr_locked_matched_left = [j for j,x,l in zip(it.count(),substr_locked_i,locked) if l=='1' and x=='(']
            if len(substr_locked_matched_left) >= count:
                continue
            else:
                substr_unlocked_matched_left = [j for j,x,l in zip(it.count(),substr_locked_i,locked) if l=='0']
                if substr_unlocked_matched_left:
                    for j in substr_unlocked_matched_left:
                        locked = locked[:j] + '1' + locked[j+1:]
                        s = s[:j] + '(' + s[j+1:]
                        result = self.matchRightLocked(s, locked)
                        for result_item in result:
                            if result_item:
                                s, locked = result_item
                                yield s, locked
                            else:
                                yield False
                    else:
                        yield False
        yield False


    def matchLeftLocked(self, s, original_locked):
        locked = original_locked
        locked_left = [i for i,x,l in zip(it.count(),s,locked) if l == '1' and x == '(']
        for count, locked_i in enumerate(locked_left,start=1):
            substr_locked_i = s[:locked_i]
            substr_locked_matched_right = [j for j,x,l in zip(it.count(),substr_locked_i,locked) if l=='1' and x=='(']
            if substr_locked_matched_right:
                continue
            else:
                substr_unlocked_matched_right = [j for j,x,l in zip(it.count(),substr_locked_i,locked) if l=='0']
                if substr_unlocked_matched_right:
                    for j in substr_unlocked_matched_right:
                        locked = locked[:j] + '1' + locked[j+1:]
                        s = s[:j] + ')' + s[j+1:]
                        result = self.matchLeftLocked(s, locked)
                        if result:
                            s,locked = result
                            break
                    else:
                        return False
        return s,locked
                            

    def canBeValid(self, s: str, locked: str) -> bool:
        if len(s) % 2 == 1:
            return False
        original_locked = locked
        original_s = s
        s,match_right = self.matchRightLocked(s,locked)
        if not match_right:
            return False
        else:
            match_left = self.matchLeftLocked(s, original_locked)
            if not match_left:
                return False
            else:
                return True

        

sol = Solution()
# st = "())(()(()(())()())(())((())(()())((())))))(((((((())(()))))("
# lo = "100011110110011011010111100111011101111110000101001101001111"
# print(sol.canBeValid(st,lo))

st = "))()))"
lo = "010100"
# print(sol.canBeValid(st,lo))
st = "(()))))("
lo = "01001111"
print(sol.canBeValid(st,lo))
# stmin = "(()))))("
# lomin = "01001111"
# print(sol.canBeValid(stmin,lomin))