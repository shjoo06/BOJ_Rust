use std::io::{self, Read};

struct Scanner {
    buf: Vec<String>,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read stdin");

        let buf = input.split_whitespace().rev().map(str::to_string).collect();

        Scanner { buf }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        let token = self.buf.pop().expect("No token left");
        token.parse().ok().expect("Failed to parse token")
    }
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.next();
    let mut blocks = vec![0; n];
    for i in 0..n {
        blocks[i] = sc.next();
    }
    let max_diff = blocks.iter().sum();
    let mut dp = vec![-1i32; max_diff + 1];
    // dp[i]: 두 탑의 높이 차이가 i일 때, 높은 탑 쪽의 최대 높이
    dp[0] = 0;

    for &bh in &blocks {
        let mut new_dp = dp.clone();
        for diff in 0..=max_diff {
            if dp[diff] == -1 { continue; }
            let high = dp[diff] as usize;
            let low = high - diff;

            let (new_high, new_diff) = (high+bh, diff+bh);
            new_dp[new_diff] = new_dp[new_diff].max(new_high as i32);

            let new_low = low + bh;
            if new_low > high {
                let new_diff = new_low - high;
                new_dp[new_diff] = new_dp[new_diff].max(new_low as i32);
            } else {
                let new_diff = high - new_low;
                new_dp[new_diff] = new_dp[new_diff].max(high as i32);
            }
        }
        dp = new_dp;
    }

    if dp[0] <= 0 {
        println!("-1");
    } else {
        println!("{}", dp[0]);
    }
}