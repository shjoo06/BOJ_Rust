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
    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        scan!(sc, a: usize, b: usize, c: i64);
        edges.push((a-1, b-1, c));
    }

    let mut dist: Vec<i64> = vec![i64::MAX; n];
    dist[0] = 0;

    for _ in 0..(n-1) {
        for (u, v, w) in &edges {
            if dist[*u] != i64::MAX && dist[*u] + w < dist[*v] {
                dist[*v] = dist[*u] + w;
            }
        }
    }
    // check for negative cycle
    for (u, v, w) in &edges {
        if dist[*u] != i64::MAX && dist[*u] + w < dist[*v] {
            writeln!(out, "{}", -1).unwrap();
            return;
        }
    }
    //
    for d in dist.into_iter().skip(1) {
        if d == i64::MAX {
            writeln!(out, "{}", -1).unwrap();
        } else {
            writeln!(out, "{}", d).unwrap();
        }
    }
}