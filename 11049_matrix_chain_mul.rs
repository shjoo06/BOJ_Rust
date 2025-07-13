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

macro_rules! scan {
    ($sc:expr, $($name:ident : $t:ty),+ $(,)?) => {
        $( let $name: $t = $sc.next(); )+
    };
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(io::stdout().lock());

    scan!(sc, n: usize);
    let mut d = vec![0usize; n+1]; // index: 0(unused) ..= n
    for i in 0..n {
        scan!(sc, r: usize, c: usize);
        if i == 0 { d[i] = r; }
        d[i+1] = c;
    }

    let mut dp = vec![vec![usize::MAX; n+1]; n+1];
    // dp[i][j] = min number of ops for multiplying matrix subchain A_i ~ A_j (i, j: 1-based indexing)
    for i in 1..=n { dp[i][i] = 0; } // base case
    for l in 2..=n { // l: length of the subchain
        for i in 1..=(n-l+1) { // i: start of the subchain
            let j = i + l - 1; // j: end of "
            for k in i..=(j-1) {
                let q = dp[i][k] + dp[k+1][j] + d[i-1]*d[k]*d[j];
                if q < dp[i][j] { dp[i][j] = q; } // update min
            }
        }
    }
    writeln!(out, "{}", dp[1][n]).unwrap();
}