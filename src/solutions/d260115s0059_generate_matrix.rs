pub struct Solution;

impl Solution {
    pub fn generate_matrix(n: i32) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut res = vec![vec![0; n]; n];
        let mut start_x = 0;
        let mut start_y = 0;
        let mut loop_count = n / 2;
        let mid = n / 2;
        let mut count = 1;
        let mut offset = 1;

        while loop_count > 0 {
            let mut i = start_x;
            let mut j = start_y;

            // 模拟填充上行从左到右 (左闭右开)
            // 使用 range: start_y .. (n - offset)
            for temp_j in j..(n - offset) {
                res[start_x][temp_j] = count;
                count += 1;
                j = temp_j; 
            }
            j += 1; // 修正到拐角位置

            // 模拟填充右列从上到下 (左闭右开)
            for temp_i in i..(n - offset) {
                res[temp_i][j] = count;
                count += 1;
                i = temp_i;
            }
            i += 1; // 修正到拐角位置

            // 模拟填充下行从右到左 (左闭右开)
            // 使用 .rev() 实现反向遍历
            for temp_j in (start_y + 1..=j).rev() {
                res[i][temp_j] = count;
                count += 1;
                j = temp_j;
            }
            j -= 1; // 修正到左下角起跳点

            // 模拟填充左列从下到上 (左闭右开)
            for temp_i in (start_x + 1..=i).rev() {
                res[temp_i][start_y] = count;
                count += 1;
            }

            // 准备进入下一圈
            start_x += 1;
            start_y += 1;
            offset += 1;
            loop_count -= 1;
        }

        // 如果 n 为奇数，填补中心点
        if n % 2 == 1 {
            res[mid][mid] = count;
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_59() {
        assert_eq!(Solution::generate_matrix(3), vec![
            vec![1, 2, 3],
            vec![8, 9, 4],
            vec![7, 6, 5]
        ]);
        assert_eq!(Solution::generate_matrix(1), vec![vec![1]]);
    }
}

