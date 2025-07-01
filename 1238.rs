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

    scan!(sc, n: usize, m: usize, x: usize);
    let mut adj_list = vec![vec![]; n];
    let mut rev_adj_list = vec![vec![]; n];
    for _ in 0..m {
        scan!(sc, u: usize, v: usize, t: usize);
        adj_list[u-1].push((v-1, t));
        rev_adj_list[v-1].push((u-1, t));
    }
    let mut dist1: Vec<usize> = vec![usize::MAX; n];
    let mut dist2: Vec<usize> = vec![usize::MAX; n];
    let mut heap1: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    let mut heap2: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    dist1[x-1] = 0;
    dist2[x-1] = 0;
    heap1.push(Reverse((0, x-1)));
    heap2.push(Reverse((0, x-1)));

    while let Some(Reverse((d, u))) = heap1.pop() {
        if dist1[u] < d { continue; }
        for &(v, w) in &adj_list[u] {
            if dist1[u] + w < dist1[v] {
                dist1[v] = dist1[u] + w;
                heap1.push( Reverse((dist1[v], v)) );
            }
        }
    }
    while let Some(Reverse((d, u))) = heap2.pop() {
        if dist2[u] < d { continue; }
        for &(v, w) in &rev_adj_list[u] {
            if dist2[u] + w < dist2[v] {
                dist2[v] = dist2[u] + w;
                heap2.push( Reverse((dist2[v], v)) );
            }
        }
    }
    let max_total_dist = dist1.iter().zip(&dist2)
        .map(|(a, b)| a + b).max().unwrap(); // Safety: no usize::MAX left in given case
    writeln!(out, "{}", max_total_dist).unwrap();
}