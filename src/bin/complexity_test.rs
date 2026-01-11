use std::io::{self, Write};
use std::time::Instant;

// O(n)
fn function1(n: u64) -> u64 {
    let mut k: u64 = 0;
    for _ in 0..n {
        k += 1;
    }
    k
}

// O(n^2)
fn function2(n: u64) -> u64 {
    let mut k: u64 = 0;
    for _ in 0..n {
        for _ in 0..n {
            k += 1;
        }
    }
    k
}

// O(n log n)
fn function3(n: u64) -> u64 {
    let mut k: u64 = 0;
    for _ in 0..n {
        let mut j = 1;
        while j < n {
            k += 1;
            j *= 2;
        }
    }
    k
}
// cargo run --package code_camp_daily_tracing_rust --bin complexity_test 
fn main() {
    loop {
        print!("输入 n (输入 0 退出): ");
        // 确保提示文字立即显示
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("读取失败");
        
        let n: u64 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("请输入有效的数字");
                continue;
            }
        };

        if n == 0 { break; }

        // 开始计时
        let start = Instant::now();

        // 测试时，手动切换注释掉不需要的函数
        // let result = function1(n);
        // let result = function2(n);
        let result = function3(n);

        let duration = start.elapsed();

        // 打印结果（result 的使用确保了循环不会被编译器完全优化掉）
        println!("结果: {}, 耗时: {:?} ({:?} ms)", 
                 result, duration, duration.as_millis());
        println!("------------------------------------");
    }
}