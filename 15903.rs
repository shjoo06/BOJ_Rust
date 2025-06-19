use std::collections::BinaryHeap;
use std::cmp::Reverse;
use std::io::{self, BufRead, Write};

fn main() {

    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let m: i64 = lines.next().unwrap().unwrap()
        .trim().split_whitespace()
        .skip(1).next().unwrap().parse().unwrap();
    let cards: Vec<Reverse<i64>> = lines.next().unwrap().unwrap().trim().split_whitespace().map(|s| Reverse(s.parse().unwrap())).collect();

    let mut min_heap = BinaryHeap::from(cards);
    for _ in 0..m {
        let min_value_1 = min_heap.pop().unwrap();
        let min_value_2 = min_heap.pop().unwrap();
        let sum = min_value_1.0 + min_value_2.0;
        min_heap.push(Reverse(sum));
        min_heap.push(Reverse(sum));
    }
    let total: i64 = min_heap.into_iter().map(|r| r.0).sum();
    writeln!(out, "{}", total);
}
