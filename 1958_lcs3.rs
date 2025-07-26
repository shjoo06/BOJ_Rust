use std::cmp::max;
use std::io::{self, BufRead, Write};

struct Scanner<R> {
    lines: io::Lines<R>,
    buf: Vec<String>,
    pos: usize,
}

impl<R: BufRead> Scanner<R> {
    fn new(reader: R) -> Self {
        Scanner {
            lines: reader.lines(),
            buf: Vec::new(),
            pos: 0,
        }
    }

    fn refill(&mut self) {
        while self.pos >= self.buf.len() {
            let line = self.lines.next().unwrap().unwrap();
            self.buf = line.split_whitespace().map(str::to_string).collect();
            self.pos = 0;
        }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.refill();
        let token = &self.buf[self.pos];
        self.pos += 1;
        token.parse().ok().expect("Failed parse")
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(io::stdout().lock());

    let s1: Vec<char> = sc.next::<String>().chars().collect();
    let s2: Vec<char> = sc.next::<String>().chars().collect();
    let s3: Vec<char> = sc.next::<String>().chars().collect();
    let (n, m, o) = (s1.len() as usize, s2.len() as usize, s3.len() as usize);
    let mut dp = vec![vec![vec![0; o+1]; m+1]; n+1];
    for i in 1..n+1 {
        for j in 1..m+1 {
            for k in 1..o+1 {
                if s1[i-1] == s2[j-1] && s2[j-1] == s3[k-1] {
                    dp[i][j][k] = dp[i-1][j-1][k-1] + 1;
                } else {
                    dp[i][j][k] = dp[i-1][j][k]
                        .max(dp[i][j-1][k])
                        .max(dp[i][j][k-1]);
                }
            }
        }
    }
    writeln!(out, "{}", dp[n][m][o]).unwrap();
}