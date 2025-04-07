/*
 * @lc app=leetcode id=4 lang=rust
 *
 * [4] Median of Two Sorted Arrays
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        // 合并两个有序数组
        let mut merged = Vec::new();
        let mut i = 0;
        let mut j = 0;
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merged.push(nums1[i]);
                i += 1;
            } else {
                merged.push(nums2[j]);
                j += 1;
            }
        }
        // 将剩余的元素添加到合并后的数组中
        while i < nums1.len() {
            merged.push(nums1[i]);
            i += 1;
        }
        while j < nums2.len() {
            merged.push(nums2[j]);
            j += 1;
        }

        // 计算中位数
        let len = merged.len();
        if len % 2 == 0 {
            (merged[len / 2 - 1] + merged[len / 2]) as f64 / 2.0
        } else {
            merged[len / 2] as f64
        }
    }
}
// @lc code=end
