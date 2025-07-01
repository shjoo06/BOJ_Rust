use std::io::{self, BufRead, Write};
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

    scan!(sc, nv: usize, ne: usize);
    scan!(sc, s: usize);
    let mut edges = Vec::with_capacity(ne);
    let mut adj_list = vec![vec![]; nv];
    for _ in 0..ne {
        scan!(sc, u: usize, v: usize, w: usize);
        edges.push((u-1, v-1, w));
        adj_list[u-1].push((v-1, w));
    }

    let mut dist: Vec<_> = vec![usize::MAX; nv];
    dist[s-1] = 0;
    let mut min_heap: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    min_heap.push( Reverse((0, s-1)) );

    while let Some(Reverse((d, u))) = min_heap.pop() {
        if d > dist[u] { continue; }
        for &(v, w) in &adj_list[u] {
            if d + w < dist[v] {
                dist[v] = d + w;
                min_heap.push( Reverse((dist[v], v)) );
            }
        }
    }

    for i in 0..nv {
        if dist[i] == usize::MAX {
            writeln!(out, "INF").unwrap();
        } else {
            writeln!(out, "{}", dist[i]).unwrap();
        }
    }
}