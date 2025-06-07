use std::collections::VecDeque;
use std::io::{self, BufRead};

fn parse_input() -> Result<(usize, usize, Vec<Vec<u8>>), Box<dyn std::error::Error>> {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let mut iter = buf.split_whitespace();
    let N: usize = iter.next().unwrap().parse().unwrap();
    let M: usize = iter.next().unwrap().parse().unwrap();

    let mut map: Vec<Vec<u8>> = Vec::with_capacity(N);
    for _ in 0..N {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let row = line.trim().bytes().map(|b| b - b'0').collect::<Vec<u8>>();
        map.push(row);
    }
    Ok((M, N, map))
}

fn main() {
    let (M, N, map) = parse_input().unwrap();
    let mut dist: Vec<Vec<Vec<i32>>> = vec![vec![vec![-1; M]; N]; 2]; // (state, x, y)
    let dirs: [(i32, i32); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let mut queue = VecDeque::from([(0, 0, 0)]);
    dist[0][0][0] = 1;

    while let Some((state, x, y)) = queue.pop_front() {
        if x == N-1 && y == M-1 {
            println!("{}", dist[state][x][y]);
            return;
        }
        for (dx, dy) in dirs {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 || nx >= N as i32 || ny >= M as i32 {
                continue;
            }
            let (nx, ny) = (nx as usize, ny as usize);
            if map[nx][ny] == 0 && dist[state][nx][ny] == -1 {
                dist[state][nx][ny] = dist[state][x][y] + 1;
                queue.push_back((state, nx, ny));
            } else if map[nx][ny] == 1 && state == 0 && dist[state+1][nx][ny] == -1 {
                dist[state+1][nx][ny] = dist[state][x][y] + 1;
                queue.push_back((state+1, nx, ny));
            }
        }
    }
    println!("-1");
}
