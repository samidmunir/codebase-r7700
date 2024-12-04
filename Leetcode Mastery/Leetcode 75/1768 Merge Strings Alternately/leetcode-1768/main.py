"""
    Leetcode 75
    - 1768. Merge Strings Alternately *EASY* (Python)

    You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

    Return the merged string.

    Example 1:
        Input: word1 = 'abc', word2 = 'pqr'
        Output: 'apbqcr'

    Example 2:
        Input: word1 = 'ab', word2 = 'pqrs'
        Output: 'apbqrs'

    Example 3:
        Input: word1 = 'abcd', word2 = 'pq'
        Output: 'apbqcd'
"""
from solution import Solution

def main():
    print('\nLeetcode #1768\n')
    """
        Test case 1
    """
    word1 = 'abc'
    word2 = 'pqr'
    output = Solution(word1 = word1, word2 = word2).get_output()
    print('- Test Case 1 -->')
    print(f'word1 = {word1}\nword2 = {word2}\noutput = {output}')

    """
        Test case 2
    """
    word1 = 'abc'
    word2 = 'pqrs'
    output = Solution(word1 = word1, word2 = word2).get_output()
    print('\n- Test Case 2 -->')
    print(f'word1 = {word1}\nword2 = {word2}\noutput = {output}')

    """
        Test case 3
    """
    word1 = 'abcd'
    word2 = 'pq'
    output = Solution(word1 = word1, word2 = word2).get_output()
    print('\n- Test Case 3 -->')
    print(f'word1 = {word1}\nword2 = {word2}\noutput = {output}')

if __name__ == '__main__':
    main()