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

fn reconstruct(
    s1: &Vec<char>,
    s2: &Vec<char>,
    dp: &Vec<Vec<usize>>,
    i: usize,
    j: usize,
    lcs: &mut Vec<char>,
) {
    if dp[i][j] == 0 {
        return;
    }
    if s1[i - 1] == s2[j - 1] {
        lcs.push(s1[i - 1]);
        reconstruct(s1, s2, dp, i - 1, j - 1, lcs);
    } else if dp[i][j] == dp[i - 1][j] {
        reconstruct(s1, s2, dp, i - 1, j, lcs);
    } else {
        reconstruct(s1, s2, dp, i, j - 1, lcs);
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(io::stdout().lock());

    let s1: Vec<char> = sc.next::<String>().chars().collect();
    let s2: Vec<char> = sc.next::<String>().chars().collect();
    let (n, m) = (s1.len() as usize, s2.len() as usize);
    let mut dp = vec![vec![0; m + 1]; n + 1];
    for i in 1..n + 1 {
        for j in 1..m + 1 {
            if s1[i - 1] == s2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]);
            }
        }
    }
    writeln!(out, "{}", dp[n][m]).unwrap();

    let mut lcs = Vec::with_capacity(dp[n][m]);
    reconstruct(&s1, &s2, &dp, n, m, &mut lcs);
    for c in lcs.into_iter().rev() {
        write!(out, "{}", c).unwrap();
    }
}
