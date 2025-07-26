use std::cmp::min;
use std::io::{self, Read};

struct Scanner {
    buf: Vec<String>,
}

impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin()
            .read_to_string(&mut input)
            .expect("Failed to read stdin");

        let buf = input.split_whitespace().rev().map(str::to_string).collect();

        Scanner { buf }
    }

    fn next<T: std::str::FromStr>(&mut self) -> T {
        let token = self.buf.pop().expect("No token left");
        token.parse().ok().expect("Failed to parse token")
    }
}

///Return minimum needed # of early adapters in subtree
///arguments: (parent of root, root node index, state, dp, adj)
fn get_subtree_min(
    p: usize,
    r: usize,
    s: usize,
    dp: &mut Vec<Vec<i32>>,
    adj_list: &Vec<Vec<usize>>,
) -> i32 {
    if dp[r][s] != -1 {
        return dp[r][s];
    }
    let mut needed = if s == 0 { 0 } else { 1 }; // count myself
    for &nx in &adj_list[r] {
        if nx == p {
            continue;
        }
        if s == 0 {
            // root is not early adapter -> all children should be early adapter in this case
            needed += get_subtree_min(r, nx, 1, dp, adj_list);
        } else {
            // children can be either early adapter(y=1) or not(y=0)
            needed += min(
                get_subtree_min(r, nx, 1, dp, adj_list),
                get_subtree_min(r, nx, 0, dp, adj_list),
            );
        }
    }
    dp[r][s] = needed;
    needed
}

fn main() {
    let mut sc = Scanner::new();
    let n: usize = sc.next();
    let mut adj_list = vec![vec![]; n + 1];
    for _ in 1..n {
        let u: usize = sc.next();
        let v: usize = sc.next();
        adj_list[u].push(v);
        adj_list[v].push(u);
    }
    let mut dp = vec![vec![-1; 2]; n + 1]; // dp[i][j] <- i: node index, j: state (y=1 or y=0)
    let result = min(
        get_subtree_min(0, 1, 0, &mut dp, &adj_list),
        get_subtree_min(0, 1, 1, &mut dp, &adj_list),
    );
    println!("{}", result);
}
