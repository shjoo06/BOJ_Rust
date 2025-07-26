use std::cmp::max;
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
    let mut dp = vec![vec![-1i32; max_diff + 1]; n + 1];
    // dp[i][j] = i번째 블록까지 고려했을 때, 두 탑의 높이 차이가 j일 때 큰 탑의 최대 높이.
    dp[0][0] = 0;

    for i in 1..=n {
        let bh = blocks[i - 1]; // 이번에 추가할 블록의 높이
        let mut dp_i = dp[i - 1].clone(); // fallback is dp[i-1] (이번 블록을 사용하지 않는 경우)
        for diff in 0..=max_diff {
            if dp[i - 1][diff] == -1 {
                continue;
            }
            // 기존 높은 & 낮은 탑의 높이 상태 (이번 블록 추가 전)
            let high = dp[i - 1][diff] as usize;
            let low = high - diff;

            // (1) 이번 블록을 더 높은 쪽에 추가하는 경우
            let new_high = high + bh;
            let new_diff = diff + bh;
            dp_i[new_diff] = dp_i[new_diff].max(new_high as i32);

            // (2) 이번 블록을 더 낮은 쪽에 추가하는 경우
            let new_low = low + bh;
            if new_low > high {
                let new_diff = new_low - high;
                dp_i[new_diff] = max(new_low as i32, dp_i[new_diff]);
            } else {
                let new_diff = high - new_low;
                dp_i[new_diff] = max(high as i32, dp_i[new_diff]);
            }
        }
        dp[i] = dp_i;
    }
    if dp[n][0] <= 0 {
        println!("-1");
    } else {
        println!("{}", dp[n][0]);
    }
}
