"""
     Leetcode 75
    - 1768. Merge Strings Alternately *EASY* (Solution - Python)

    Runtime: 38 ms
    Memory 17.24 MB
"""

class Solution:
    def __init__(self, word1: str, word2: str) -> None:
        self.word1 = word1
        self.word2 = word2
        self.merged_word = self.mergeAlternately(word1 = self.word1, word2 = self.word2)
    
    def mergeAlternately(self, word1: str, word2: str) -> str:
        merged = ''

        if len(word1) >= len(word2):
            for i in range(len(word2)):
                merged += word1[i]
                merged += word2[i]
            merged += word1[len(word2): len(word1)]
        else:
            for i in range(len(word1)):
                merged += word1[i]
                merged += word2[i]
            merged += word2[len(word1): len(word2)]
            
        return merged
    
    def get_output(self) -> str:
        return self.merged_word