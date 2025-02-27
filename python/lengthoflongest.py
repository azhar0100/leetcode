def lengthOfLongestSubstring(self, s: str) -> int:
        starting_j = 0
        repeating = False
        max_len = 0
        substr = ""
        for i in range(1,len(s)):
            substr = s[starting_j:i]
            n = len(substr)
            repeating = len(substr) > len(set(substr))
            if repeating:
                new_len = n - 1
                if new_len > max_len:
                    max_len = new_len
                starting_j = i - 1
            
        return max_len

print(lengthOfLongestSubstring(None, "pwwkew"))
print(lengthOfLongestSubstring(None, "abcabcbb"))