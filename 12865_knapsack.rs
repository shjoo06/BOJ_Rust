use std::io::{self, Read, Write};
use std::cmp::max;

struct Scanner {
    buf: Vec<String>,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input)
            .expect("Failed to read stdin");

        let buf = input
            .split_whitespace()
            .rev()
            .map(str::to_string)
            .collect();

        Scanner { buf }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        let token = self.buf.pop()
            .expect("No token left");
        token.parse()
            .ok()
            .expect("Failed to parse token")
    }
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = io::BufWriter::new(io::stdout().lock());

    let n: usize = sc.next();
    let k: usize = sc.next();
    let mut w = vec![0usize; n+1];
    let mut v = vec![0usize; n+1];
    for i in 1..=n {
        w[i] = sc.next();
        v[i] = sc.next();
    }
    let mut dp = vec![vec![0usize; k+1]; n+1];
    for i in 1..=n {
        for j in 1..=k {
            if (j as i32 - w[i] as i32) >= 0 {
                dp[i][j] = max(dp[i-1][j-w[i]] + v[i], dp[i-1][j]); // get i or not get i
            } else { // can't get i
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    writeln!(out, "{}", dp[n][k]).unwrap();
}