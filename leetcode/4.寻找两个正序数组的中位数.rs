/*
 * @lc app=leetcode.cn id=4 lang=rust
 *
 * [4] 寻找两个正序数组的中位数
 */

// @lc code=start
impl Solution {
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        //1、合并两个有序数组
        // 计算总长度，初始化数组
        let mut merges: Vec<i32> = Vec::with_capacity(nums1.len() + nums2.len());
        // 开始合并
        let (mut i, mut j) = (0, 0);
        // 开始合并数组
        while i < nums1.len() && j < nums2.len() {
            if nums1[i] < nums2[j] {
                merges.push(nums1[i]);
                i += 1;
            } else {
                merges.push(nums2[j]);
                j += 1;
            }
        }
        // 如果 nums1 还有剩余
        merges.extend_from_slice(&nums1[i..]);
        // 如果 nums2 还有剩余
        merges.extend_from_slice(&nums2[j..]);
        //2、 计算中位数
        let len = merges.len();
        if len % 2 == 0 {
            // 偶数个元素，中位数是中间两个数的平均值
            let mid1 = merges[len / 2 - 1];
            let mid2 = merges[len / 2];
            (mid1 + mid2) as f64 / 2.0
        } else {
            // 奇数个元素，中位数是中间的那个数
            merges[len / 2] as f64
        }
    }
}
// @lc code=end
