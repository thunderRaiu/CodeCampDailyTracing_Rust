pub struct Solution;

impl Solution {
    pub fn sorted_squares_brute(mut nums: Vec<i32>) -> Vec<i32> {
        for x in nums.iter_mut() {
            *x = (*x) * (*x);
        }
        nums.sort();
        nums
    }
    pub fn sorted_squares(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n]; // 预分配空间
        let (mut left, mut right) = (0, n - 1);
        let mut index = n - 1;

        // 使用 while 循环配合 usize 索引
        // 注意：由于 right 是 usize，当 n=0 时需要额外处理，
        // 但 LeetCode 题目保证 nums.length >= 1
        while left <= right {
            let l_sq = nums[left] * nums[left];
            let r_sq = nums[right] * nums[right];

            if l_sq > r_sq {
                result[index] = l_sq;
                left += 1;
            } else {
                result[index] = r_sq;
                // 这里的判断是为了防止 usize 下溢 (0 - 1)
                if right > 0 {
                    right -= 1;
                } else {
                    break; // 如果 right 已经到 0 且 left 也处理完了，跳出
                }
            }
            
            if index > 0 {
                index -= 1;
            } else {
                break; // 填满了 result
            }
        }
        result
    }

    pub fn sorted_squares_functional(nums: Vec<i32>) -> Vec<i32> {
        let mut res: Vec<i32> = nums.into_iter().map(|x| x * x).collect();
        res.sort_unstable(); // sort_unstable 通常比 sort 快
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
// cargo test --package code_camp_daily_tracing_rust --lib -- solutions::d260113s0977_sorted_squres::tests::test_977 --exact --nocapture 
    #[test]
    fn test_977() {
        assert_eq!(Solution::sorted_squares(vec![-4, -1, 0, 3, 10]), vec![0, 1, 9, 16, 100]);
        assert_eq!(Solution::sorted_squares_functional(vec![-7, -3, 2, 3, 11]), vec![4, 9, 9, 49, 121]);
    }
}