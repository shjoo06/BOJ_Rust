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

    let mut dp = vec![false; n+1];
    dp[0] = true;
    for i in 1..=n {
        if !dp[i-1] { dp[i] = true; }
        if i>2 && !dp[i-3] {
            dp[i] = true;
        }
        if i>3 && !dp[i-4] {
            dp[i] = true;
        }
    }
    if dp[n] {
        writeln!(out, "{}", "SK").unwrap();
    } else {
        writeln!(out, "{}", "CY").unwrap();
    }
}