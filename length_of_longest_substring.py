class Solution:
    def lengthOfLongestSubstring(self, s: str) -> int:
        long_set = set()
        long_len, slide_index, s_len = 0, 0, len(s)

        for i in range(s_len):
            if i > 0:
                long_set.remove(s[i - 1])

            # 窗口滑动从最左开始
            while (slide_index < s_len) and (s[slide_index] not in long_set):
                long_set.add(s[slide_index])
                slide_index += 1
                continue

            long_len = max(long_len, len(long_set))
        return max(len(long_set), long_len)
