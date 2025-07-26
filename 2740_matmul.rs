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

    scan!(sc, n: usize, m: usize);
    let mut A: Vec<Vec<i32>> = vec![vec![0; m]; n];
    for i in 0..n {
        for j in 0..m {
            A[i][j] = sc.next();
        }
    }
    scan!(sc, m: usize, k: usize);
    let mut B: Vec<Vec<i32>> = vec![vec![0; k]; m];
    for i in 0..m {
        for j in 0..k {
            B[i][j] = sc.next();
        }
    }

    let mut res: Vec<Vec<i32>> = vec![vec![0; k]; n];
    for i in 0..n {
        for j in 0..m {
            for l in 0..k {
                res[i][l] += A[i][j] * B[j][l];
            }
        }
    }
    for elem in res {
        for c in elem {
            write!(out, "{} ", c).unwrap();
        }
        write!(out, "\n").unwrap();
    }
}