/*
 * @lc app=leetcode.cn id=42 lang=rust
 *
 * [42] 接雨水
 */

// @lc code=start
impl Solution {
    pub fn trap(height: Vec<i32>) -> i32 {
        height
            .iter()
            .enumerate()
            .fold(0i32, |mut sum, (idx, &item)| {
                let lm = *height[..=idx].iter().max().unwrap_or(&0);
                let rm = *height[idx..].iter().max().unwrap_or(&0);
                let min = *[lm, rm].iter().min().unwrap();
                sum += min - item;
                return sum;
            })
    }
}
// @lc code=end
