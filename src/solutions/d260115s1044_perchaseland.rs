use std::io::{self, BufRead};
use std::cmp;

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // 1. 读取 n 和 m
    let first_line = lines.next().unwrap().unwrap();
    let dims: Vec<usize> = first_line
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    let (n, m) = (dims[0], dims[1]);

    let mut matrix = vec![vec![0; m]; n];
    let mut total_sum = 0;

    // 2. 读取矩阵并计算总和
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let row: Vec<int> = line
            .split_whitespace()
            .map(|s| {
                let val = s.parse::<i32>().unwrap();
                total_sum += val;
                val
            })
            .collect();
        matrix[i] = row;
    }

    let mut result = i32::MAX;

    // 3. 按行切割
    // 我们不需要真的去算每一行的前缀和数组，只需要一个变量累加即可
    let mut row_sum = 0;
    for i in 0..n {
        for j in 0..m {
            row_sum += matrix[i][j];
        }
        // 切割位置是在第 i 行之后
        // 第一部分的和是 row_sum, 第二部分的和是 total_sum - row_sum
        // 差值是 abs(total_sum - row_sum - row_sum)
        let diff = (total_sum - 2 * row_sum).abs();
        result = cmp::min(result, diff);
    }

    // 4. 按列切割
    let mut col_sum = 0;
    for j in 0..m {
        for i in 0..n {
            col_sum += matrix[i][j];
        }
        let diff = (total_sum - 2 * col_sum).abs();
        result = cmp::min(result, diff);
    }

    println!("{}", result);
}