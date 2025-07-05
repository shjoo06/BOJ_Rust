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
    let mut p = vec![0usize; n+1];
    for i in 1..=n {
        p[i] = sc.next();
    }

    let mut dp = vec![0usize; n+1]; // stores largest M when card_num=i (1->i->N)
    for i in 1..=n {
        for j in 1..=i {
            if dp[i] < p[j] + dp[i-j] {
                dp[i] = p[j] + dp[i-j];
            }
        }
    }
    writeln!(out, "{}", dp[n]).unwrap();
}