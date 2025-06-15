use std::io::{self, BufRead};
use std::collections::VecDeque;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // input parsing
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let first_line = lines.next().unwrap()?;
    let mut nm_iter = first_line.trim().split_whitespace();
    let n: usize = nm_iter.next().unwrap().parse()?;
    let m: usize = nm_iter.next().unwrap().parse()?;
    let mut grid: Vec<Vec<char>> = Vec::with_capacity(m);
    for _ in 0..m {
        let row: Vec<char> = lines.next().unwrap()?.trim().chars().collect();
        grid.push(row);
    }

    let mut items = Vec::with_capacity(5);
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for i in 0..m {
        for j in 0..n {
            match grid[i][j] {
                'S' => { start = (i, j); },
                'X' => { items.push((i, j)); },
                'E' => { end = (i, j); },
                _ => {}
            }
        }
    }
    let is_all_packed = (1 << items.len()) -1;

    // initialize
    let mut dist: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; n]; m]; is_all_packed+1]; // (state, y, x)
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue = VecDeque::from([(0usize, start.0, start.1)]);
    dist[0][start.0][start.1] = 0;

    // bfs
    // state is represented as bit masking
    while let Some((state, y, x)) = queue.pop_front() {
        if (y, x) == end && state == is_all_packed {
            println!("{}", dist[state][y][x]);
            return Ok(());
        }
        for (dx, dy) in dirs {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 || nx >= n as i32 || ny >= m as i32 { // grid boundary check
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            let c = grid[ny][nx];
            if c == '#' {
                continue;
            }

            if dist[state][ny][nx] == -1 { // 해당 state로 해당 위치에 방문한 적 없음
                let mut new_state = state;
                if c == 'X' {
                    let item_index = items.iter().position(|&(item_y, item_x)| (ny, nx) == (item_y, item_x)).unwrap();
                    new_state |= (1 << item_index);
                }
                dist[new_state][ny][nx] = dist[state][y][x] + 1;
                queue.push_back((new_state, ny, nx));
            }
        }
    }
    println!("-1");
    Ok(())
}