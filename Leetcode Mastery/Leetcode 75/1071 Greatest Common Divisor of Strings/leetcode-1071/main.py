"""
    Leetcode 75
    - 1071. Greatest Common Divisor of Strings

    For two strings s and t, we say "t divides s" if an only if s = t + t + t + ... + t + t (i,e., it is concatenated with itself one or more times).

    Given two strings str1 and str2, return the largest string x such that x divides both str1 and str2.

    Example 1:
        Input: str1 = 'ABCABC', str2 = 'ABC'
        Output: 'ABC'

    Example 2:
        Input: str1 = 'ABABAB', str2 = 'ABAB'
        Output: 'AB'

    Example 3:
        Input: str1 = 'LEET', str2 = 'CODE'
        Output: ''
"""
from solution import Solution

def main():
    print('\nLeetcode #1071\n')

    """
        Test case 1
    """
    print('- Test Case 1 -->')
    str1 = 'ABCABC'
    str2 = 'ABC'
    output = Solution(str1 = str1, str2 = str2).get_output()
    print(f'str1 = {str1}\nstr2 = {str2}\noutput = {output}')

    """
        Test case 2
    """
    print('\n- Test Case 2 -->')
    str1 = 'ABABAB'
    str2 = 'ABAB'
    output = Solution(str1 = str1, str2 = str2).get_output()
    print(f'str1 = {str1}\nstr2 = {str2}\noutput = {output}')

    """
        Test case 3
    """
    str1 = 'LEET'
    str2 = 'CODE'
    output = Solution(str1 = str1, str2 = str2).get_output()
    print('\n- Test Case 3 -->')
    print(f'str1 = {str1}\nstr2 = {str2}\noutput = {output}')

if __name__ == '__main__':
    main()