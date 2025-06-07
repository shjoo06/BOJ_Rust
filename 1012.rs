use std::collections::VecDeque;
use std::io::{self, BufRead};

fn parse_input() -> Result<Vec<(usize, usize, Vec<(usize, usize)>)>, Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let t: usize = lines.next().ok_or("No input")??.parse()?;
    let mut test_cases = Vec::with_capacity(t);

    for _ in 0..t {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let m: usize = iter.next().unwrap().parse()?;
        let n: usize = iter.next().unwrap().parse()?;
        let k: usize = iter.next().unwrap().parse()?;

        let mut positions = Vec::with_capacity(k);
        for _ in 0..k {
            let pos_line = lines.next().unwrap()?;
            let mut pos_iter = pos_line.split_whitespace();
            let x: usize = pos_iter.next().unwrap().parse()?;
            let y: usize = pos_iter.next().unwrap().parse()?;
            positions.push((x, y));
        }
        test_cases.push((m, n, positions));
    }
    Ok(test_cases)
}

fn bfs_islands(m: usize, n: usize, positions: Vec<(usize, usize)>) -> i32 {
    let mut cabbages: Vec<Vec<bool>> = vec![vec![false; n]; m];
    for (x, y) in &positions {
        cabbages[*x][*y] = true;
    }
    let mut visited: Vec<Vec<bool>> = vec![vec![false; n]; m];
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut islands = 0;

    for (x, y) in positions {
        if visited[x][y] {
            continue;
        }
        // BFS
        let mut queue = VecDeque::from([(x, y)]);
        visited[x][y] = true;
        while let Some((cx, cy)) = queue.pop_front() {
            for (dx, dy) in dirs {
                let nx = cx as i32 + dx;
                let ny = cy as i32 + dy;
                if nx < 0 || ny < 0 || nx >= m as i32 || ny >= n as i32 {
                    continue;
                }
                let (nx, ny) = (nx as usize, ny as usize);

                if cabbages[nx][ny] && !visited[nx][ny] {
                    visited[nx][ny] = true;
                    queue.push_back((nx, ny));
                }
            }
        }
        islands += 1;
    }
    islands
}

fn main() {
    let test_cases = parse_input().unwrap();
    for (m, n, pos) in test_cases {
        println!("{}", bfs_islands(m, n, pos));
    }
}


