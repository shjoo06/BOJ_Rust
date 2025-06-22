use std::io::{self, BufRead, Write};
use std::collections::VecDeque;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();

    // parse input & initialize
    let binding = lines.next().unwrap().unwrap();
    let mut nm = binding.split_whitespace();
    let n: usize = nm.next().unwrap().parse().unwrap();
    let m: usize = nm.next().unwrap().parse().unwrap();
    let mut grid = Vec::with_capacity(n);
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut dist = vec![vec![-1; m]; n]; // indexing format: (y, x)
    dist[0][0] = 1;
    for _ in 0..n {
        let line: Vec<bool> = lines.next().unwrap().unwrap().trim()
            .chars().map(|c| c == '1').collect();
        grid.push(line);
    }
    let mut queue = VecDeque::from([(0, 0)]);

    // bfs
    while let Some((y, x)) = queue.pop_front() {
        if x == m-1 && y == n-1 {
            writeln!(out, "{}", dist[y][x]).unwrap();
            return;
        }
        for (dx, dy) in dirs {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 || nx >= m as i32 || ny >= n as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if grid[ny][nx] && dist[ny][nx] == -1 {
                dist[ny][nx] = dist[y][x] + 1;
                queue.push_back((ny, nx));
            }
        }
    }
}