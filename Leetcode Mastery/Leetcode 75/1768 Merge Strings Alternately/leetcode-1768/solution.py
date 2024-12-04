"""
     Leetcode 75
    - 1768. Merge Strings Alternately *EASY* (Solution - Python)
"""

class Solution:
    def __init__(self, word1: str, word2: str) -> str:
        self.word1 = word1
        self.word2 = word2
        self.merged_word = ''