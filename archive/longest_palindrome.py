class Solution:
    def longestPalindrome(self, s: str) -> str:
        length = len(s)
        dp = [[False] * length for _ in range(length)]
        value = ""

        # l+1可表示字符串长度
        for l in range(length):
            for i in range(length):
                # i表示字串起始位置，j表示终止位置
                j = i + l

                if j + 1 > length:
                    break

                if l == 0:
                    dp[i][j] = True
                elif l == 1:
                    dp[i][j] = (s[i] == s[j])
                else:
                    dp[i][j] = (dp[i + 1][j - 1] and s[i] == s[j])

                if dp[i][j] and l + 1 > len(value):
                    value = s[i:j + 1]

        return valu
