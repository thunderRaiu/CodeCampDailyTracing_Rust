use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let mut stdout = io::BufWriter::new(io::stdout());

    let mut buf = String::new();
    
    // 读取 n
    handle.read_line(&mut buf)?;
    let n: usize = buf.trim().parse().unwrap();
    buf.clear();

    let mut p = Vec::with_capacity(n);
    let mut current_sum = 0;

    // 构建前缀和
    for _ in 0..n {
        handle.read_line(&mut buf)?;
        let val: i32 = buf.trim().parse().unwrap();
        current_sum += val;
        p.push(current_sum);
        buf.clear();
    }

    // 处理查询
    while handle.read_line(&mut buf)? > 0 {
        let mut parts = buf.split_whitespace();
        if let (Some(a_str), Some(b_str)) = (parts.next(), parts.next()) {
            let a: usize = a_str.parse().unwrap();
            let b: usize = b_str.parse().unwrap();
            let sum = if a == 0 { p[b] } else { p[b] - p[a - 1] };
            writeln!(stdout, "{}", sum)?;
        }
        buf.clear();
    }
    stdout.flush()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    // 假设你的前缀和计算逻辑封装在一个函数里
    fn get_range_sum(p: &[i32], a: usize, b: usize) -> i32 {
        if a == 0 { p[b] } else { p[b] - p[a - 1] }
    }

    #[test]
    fn test_prefix_sum_logic() {
        let vec = vec![1, 2, 3, 4, 5];
        // 构造前缀和数组 p: [1, 3, 6, 10, 15]
        let mut p = Vec::new();
        let mut sum = 0;
        for x in vec {
            sum += x;
            p.push(sum);
        }

        assert_eq!(get_range_sum(&p, 0, 1), 3);  // 区间 [0,1] -> 1+2 = 3
        assert_eq!(get_range_sum(&p, 1, 3), 9);  // 区间 [1,3] -> 2+3+4 = 9
        assert_eq!(get_range_sum(&p, 0, 4), 15); // 全选
        assert_eq!(get_range_sum(&p, 4, 4), 5);  // 只选最后一个
    }
}