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

    scan!(sc, tc: u8);
    'outer: for _ in 0..tc {
        scan!(sc, n: usize, m: usize, w: usize);
        let mut edges = Vec::with_capacity(2*m + w + n);
        for _ in 0..m {
            scan!(sc, s: usize, e: usize, t: i64);
            edges.push((s, e, t));
            edges.push((e, s, t))
        }
        for _ in 0..w {
            scan!(sc, s: usize, e: usize, t: i64);
            edges.push((s, e, -t));
        }
        for i in 1..=n {
            edges.push((0, i, 0));
        }

        let mut dist: Vec<i64> = vec![i64::MAX; n+1];
        dist[0] = 0;
        for _ in 0..n {
            for &(u, v, w) in &edges {
                if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                    dist[v] = dist[u] + w;
                }
            }
        }
        for &(u, v, w) in &edges {
            if dist[u] != i64::MAX && dist[u] + w < dist[v] {
                writeln!(out, "YES").unwrap();
                continue 'outer;
            }
        }
        writeln!(out, "NO").unwrap();
    }
}