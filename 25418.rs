use std::collections::VecDeque;
use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().expect("FI").parse().expect("FI");
    let k: i32 = iter.next().expect("FI").parse().expect("FI");

    let mut queue = VecDeque::from([a]);
    let mut dist: Vec<i32> = vec![-1; (k+1) as usize];
    dist[a as usize] = 0;
    while queue.len() != 0 {
        let x = queue.pop_front().unwrap();
        for nx in [x+1, x*2] {
            if (nx < k) && (dist[nx as usize] == -1) {
                dist[nx as usize] = dist[x as usize] + 1;
                queue.push_back(nx);
            } else if (nx == k) {
                println!("{}", dist[x as usize] + 1);
                return;
            }
        }
    }
}
