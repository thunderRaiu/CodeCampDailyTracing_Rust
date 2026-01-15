pub struct Solution;

impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut result = i32::MAX;
        let mut sum = 0;      // 滑动窗口数值之和
        let mut left = 0;     // 窗口起始位置 (i)
        
        // right 是窗口结束位置 (j)
        for right in 0..n {
            sum += nums[right];
            
            // 使用 while 寻找符合条件的最小窗口
            // 只要当前窗口和大于等于 target，就尝试缩小窗口左边界
            while sum >= target {
                let sub_length = (right - left + 1) as i32;
                if sub_length < result {
                    result = sub_length;
                }
                
                // 窗口缩小：减去左侧值，左指针右移
                sum -= nums[left];
                left += 1;
            }
        }

        // 如果 result 依然是 i32::MAX，说明没有符合条件的子数组
        if result == i32::MAX { 0 } else { result }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_209() {
        assert_eq!(Solution::min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
        assert_eq!(Solution::min_sub_array_len(4, vec![1, 4, 4]), 1);
        assert_eq!(Solution::min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }
}