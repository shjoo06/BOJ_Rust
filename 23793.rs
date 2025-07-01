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

fn dijkstra(adj_list: &Vec<Vec<(usize, usize)>>, src: usize,
            dist: &mut Vec<usize>, skip: Option<usize>) {
    let mut heap: BinaryHeap<Reverse<(usize, usize)>> = BinaryHeap::new();
    dist[src] = 0;
    heap.push(Reverse((0, src)));
    while let Some(Reverse((d, u))) = heap.pop() {
        if dist[u] < d { continue; }
        'inner: for &(v, w) in &adj_list[u] {
            if skip.map_or(false, |s| v == s) {
                continue 'inner;
            }
            if dist[u] + w < dist[v] {
                dist[v] = dist[u] + w;
                heap.push ( Reverse((dist[v], v)) );
            }
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let mut sc = Scanner::new(stdin.lock());
    let mut out = io::BufWriter::new(io::stdout().lock());

    scan!(sc, n: usize, m: usize);
    let mut adj_list = vec![vec![]; n];
    for _ in 0..m {
        scan!(sc, u: usize, v: usize, w: usize);
        adj_list[u-1].push((v-1, w));
    }
    scan!(sc, x: usize, y: usize, z: usize); // x: src, z: dst
    let (o1, o2): (i32, i32);

    let mut dist_from_x = vec![usize::MAX; n];
    let mut dist_from_y = vec![usize::MAX; n];
    dijkstra(&adj_list, x-1, &mut dist_from_x, None);
    dijkstra(&adj_list, y-1, &mut dist_from_y, None);
    if dist_from_x[y-1] == usize::MAX || dist_from_y[z-1] == usize::MAX {
        o1 = -1;
    } else {
        o1 = (dist_from_x[y-1] + dist_from_y[z-1]) as i32;
    }

    let mut dist = vec![usize::MAX; n];
    dijkstra(&adj_list, x-1, &mut dist, Some(y-1));
    if dist[z-1] == usize::MAX {
        o2 = -1;
    } else {
        o2 = dist[z-1] as i32;
    }
    writeln!(out, "{} {}", o1, o2).unwrap();
}