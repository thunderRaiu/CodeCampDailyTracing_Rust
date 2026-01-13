pub struct Solution;

impl Solution {
    // 1. 暴力法 (Brute Force)
    pub fn remove_element_brute(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut size = nums.len();
        let mut i = 0;
        while i < size {
            if nums[i] == val {
                for j in i + 1..size {
                    nums[j - 1] = nums[j];
                }
                size -= 1;
                // 注意这里不需要 i++，因为后面的元素移过来了，需要原地再次检查
                continue;
            }
            i += 1;
        }
        size as i32
    }

    // 2. 快慢指针 (Fast-Slow Pointers) - 最推荐的通用解法
    pub fn remove_element_fast_slow(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut slow = 0;
        for fast in 0..nums.len() {
            if nums[fast] != val {
                nums[slow] = nums[fast];
                slow += 1;
            }
        }
        slow as i32
    }

    // 3. 相向双指针法 (Two Pointers from both ends)
    // 这种方法改变了元素的相对顺序，但在移除元素较多时移动次数最少
    pub fn remove_element_two_way(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len() as i32 - 1;
        
        while left <= right {
            // 找到左边等于 val 的元素
            while left <= right && nums[left as usize] != val {
                left += 1;
            }
            // 找到右边不等于 val 的元素
            while left <= right && nums[right as usize] == val {
                right -= 1;
            }
            // 交换或覆盖
            if left < right {
                nums[left as usize] = nums[right as usize];
                left += 1;
                right -= 1;
            }
        }
        left as i32
    }

    // 4. 相向双指针优化版 (LeetCode 官方推荐风格)
    pub fn remove_element_optimized(nums: &mut Vec<i32>, val: i32) -> i32 {
        let mut left = 0;
        let mut right = nums.len();
        while left < right {
            if nums[left] == val {
                nums[left] = nums[right - 1];
                right -= 1;
            } else {
                left += 1;
            }
        }
        left as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_27() {
        let mut vec1 = vec![3, 2, 2, 3];
        // assert_eq!(Solution::remove_element_fast_slow(&mut vec1, 3), 2);
        // assert_eq!(Solution::remove_element_brute(&mut vec1, 3), 2);
        assert_eq!(Solution::remove_element_two_way(&mut vec1, 3), 2);
        
        let mut vec2 = vec![0, 1, 2, 2, 3, 0, 4, 2];
        assert_eq!(Solution::remove_element_optimized(&mut vec2, 2), 5);
    }
}