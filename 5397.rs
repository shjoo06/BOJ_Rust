use std::io::{self, BufRead, Write};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let mut out = io::BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    let case_num: u32 = lines.next().unwrap().unwrap().parse().unwrap();
    for _ in 0..case_num {
        let line = String::from(lines.next().unwrap().unwrap());
        let mut left: Vec<char> = Vec::new(); // left side of cursor
        let mut right: Vec<char> = Vec::new();
        for c in line.chars() {
            match c {
                '<' => if let Some(ch) = left.pop() {
                    right.push(ch);
                },
                '>' => if let Some(ch) = right.pop() {
                    left.push(ch);
                },
                '-' => { left.pop(); },
                _ => { left.push(c); },
            }
        }
        let pw: String = left.into_iter()
            .chain(right.into_iter().rev()).collect();
        writeln!(out, "{}", pw);
    }
}