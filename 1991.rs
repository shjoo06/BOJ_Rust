use std::io::{self, BufRead};

#[derive (Clone)]
struct Node {
    left: Option<usize>,
    right: Option<usize>,
}

fn preorder(tree: &Vec<Node>, idx: usize, buf: &mut String) {
    buf.push((idx as u8 + b'A') as char);
    if let Some(left) = tree[idx].left { preorder(tree, left, buf); }
    if let Some(right) = tree[idx].right { preorder(tree, right, buf); }
}

fn inorder(tree: &Vec<Node>, idx: usize, buf: &mut String) {
    if let Some(left) = tree[idx].left { inorder(tree, left, buf); }
    buf.push((idx as u8 + b'A') as char);
    if let Some(right) = tree[idx].right { inorder(tree, right, buf); }
}

fn postorder(tree: &Vec<Node>, idx: usize, buf: &mut String) {
    if let Some(left) = tree[idx].left { postorder(tree, left, buf); }
    if let Some(right) = tree[idx].right { postorder(tree, right, buf); }
    buf.push((idx as u8 + b'A') as char);
}

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut tree = vec![Node { left: None, right: None }; n];
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let temp: &[u8] = line.as_bytes();
        let pi = (temp[0] - b'A') as usize; // map A as integer index 0
        tree[pi].left = if temp[2] == b'.' { None } else { Some((temp[2] - b'A') as usize) };
        tree[pi].right = if temp[4] == b'.' { None } else { Some((temp[4] - b'A') as usize) };
    }

    let mut pre = String::with_capacity(n);
    let mut inord = String::with_capacity(n);
    let mut post = String::with_capacity(n);
    preorder(&tree, 0, &mut pre);
    inorder(&tree, 0, &mut inord);
    postorder(&tree, 0, &mut post);

    println!("{}", pre);
    println!("{}", inord);
    println!("{}", post);
}

