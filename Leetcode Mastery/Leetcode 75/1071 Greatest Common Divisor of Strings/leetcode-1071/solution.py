"""
    Leetcode 75
    - 1071. Greatest Common Divisor of Strings *EASY* (Solution - Python)

    Runtime: 0 ms
    Memory 17.34 MB
"""

class Solution:
    def __init__(self, str1: str, str2: str) -> None:
        self.str1 = str1
        self.str2 = str2
        self.x = self.gcdOfStrings(str1 = self.str1, str2 = self.str2)

    def is_divisor(self, str1, str2, len1, len2, L):
        if len1 % L or len2 % L:
            return False
        f1, f2 = len1 // L, len2 // L
        return str1[:L] * f1 == str1 and str1[:L] * f2 == str2

    def gcdOfStrings(self, str1: str, str2: str) -> str:
        len1, len2 = len(str1), len(str2)

        for L in range(min(len1, len2), 0, -1):
            if self.is_divisor(str1, str2, len1, len2, L):
                return str1[:L]
        
        return ''
    
    def get_output(self) -> str:
        return self.x