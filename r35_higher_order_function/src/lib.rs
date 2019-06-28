
pub fn execute() {
    println!("Find the sum of all the squared odd numbers under 1000");
    let upper = 1000;

    let mut acc = 0;
    for n in 0 .. {
        let n_squared = n * n;
        if n_squared >= upper {
            break;
        } else if is_odd(n_squared) {
            acc += n_squared;
        }
    }
    println!("imprative style: {}", acc);

    // 函数式的写法
    let sum_of_squared_odd_numbers: u32 =
        (0..).map(|n| n * n)             // 所有自然数取平方
            .take_while(|&n| n < upper) // 取小于上限的
            .filter(|&n| is_odd(n))     // 取奇数
            .fold(0, |sum, i| sum + i); // 最后加起来
    println!("functional style: {}", sum_of_squared_odd_numbers);
}

fn is_odd(n: u32) -> bool {
    n % 2 == 1
}

