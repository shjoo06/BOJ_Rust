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

    scan!(sc, n: usize, t: usize);
    let mut grid: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            grid[i][j] = sc.next();
        }
    }

    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut dist = vec![vec![vec![usize::MAX; n]; n]; 3]; // (state, y, x)
    let mut heap = BinaryHeap::from([ Reverse((0usize, 0usize, 0usize, 0usize)) ]); // (w, state, y, x)
    dist[0][0][0] = 0;

    while let Some(Reverse((w, state, y, x))) = heap.pop() {
        if dist[state][y][x] < w { continue; }
        // (state, y, x)와 연결된 정점들을 relax
        for (dy, dx) in dirs {
            let (ny, nx) = (y as i32 + dy, x as i32 + dx);
            if ny < 0 || ny >= n as i32 || nx < 0 || nx >= n as i32 { continue; }
            let (ny, nx) = (ny as usize, nx as usize);
            let mut nw = w + t; // w = dist[state][y][x]
            let new_state = (state + 1) % 3;
            if new_state == 0 { nw += grid[ny][nx]; }

            if nw < dist[new_state][ny][nx] {
                dist[new_state][ny][nx] = nw;
                heap.push( Reverse((nw, new_state, ny, nx)) );
            }
        }
    }
    let shortest = dist.iter().map(|one_state| one_state[n-1][n-1]).min().unwrap();
    writeln!(out, "{}", shortest).unwrap();
}