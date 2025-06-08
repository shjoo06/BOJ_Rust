use std::io::{self, BufRead};
use std::cmp::max;

fn parse_input() -> Result<Vec<Vec<(usize, i32)>>, Box<dyn std::error::Error>> {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n_nodes: usize = lines.next().ok_or("No input")??.parse()?;
    let mut adj: Vec<Vec<(usize, i32)>> = vec![vec![]; n_nodes]; // adjacency list with 0-based indexing
    for _ in 0..n_nodes-1 {
        let line = lines.next().unwrap()?;
        let mut iter = line.split_whitespace();
        let p: usize = iter.next().unwrap().parse()?;
        let c: usize = iter.next().unwrap().parse()?;
        let w: i32 = iter.next().unwrap().parse()?;
        adj[p-1].push((c-1, w));
    }
    Ok(adj)
}

fn dfs(adj: &Vec<Vec<(usize, i32)>>, u: usize, diameter: &mut i32) -> i32 {
    let mut lengths = vec![];
    for &(c, w) in &adj[u] {
        lengths.push(dfs(adj, c, diameter) + w);
    }
    lengths.sort_by(|a, b| b.cmp(a)); //in descending order
    //update tree diameter
    match lengths.len() {
        0 => return 0, // base case
        1 => *diameter = max(*diameter, lengths[0]),
        _ => *diameter = max(*diameter, lengths[0] + lengths[1]), // >1
    }
    return lengths[0]
}

fn main() {
    let mut diameter = 0;
    let adj = parse_input().unwrap();
    dfs(&adj, 0, &mut diameter);
    println!("{}", diameter);
}




