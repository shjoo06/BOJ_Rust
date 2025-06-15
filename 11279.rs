use std::io::{self, BufRead, Write};
// Max heap is available at std::collections::BinaryHeap
// re-implemented max_heap for practice

fn max_heapify(arr: &mut Vec<i32>, index: usize) {
    let mut largest = index;
    let (left, right) = (2 * index + 1, 2 * index + 2);

    if left < arr.len() && arr[left] > arr[largest] {
        largest = left;
    }
    if right < arr.len() && arr[right] > arr[largest] {
        largest = right;
    }
    if largest != index {
        arr.swap(index, largest);
        max_heapify(arr, largest);
    }
}

fn generate_max_heap(arr: &mut Vec<i32>) {
    let size = arr.len();
    for i in (0..size/2).rev() {
        max_heapify(arr, i);
    }
}

fn insert_int(arr: &mut Vec<i32>, x: i32) {
    arr.push(x);
    let mut current_index = arr.len() - 1;

    while current_index > 0 && arr[(current_index - 1) / 2] < arr[current_index] {
        let parent_index = (current_index - 1) / 2;
        arr.swap(current_index, parent_index);
        current_index = parent_index;
    }
}

fn pop_max(arr: &mut Vec<i32>) -> i32 {
    if arr.is_empty() {
        return 0; // as required in this problem
    }

    let max_value = arr[0];
    if arr.len() > 1 {
        arr[0] = arr.pop().unwrap();
        max_heapify(arr, 0);
    } else {
        arr.pop();
    }
    max_value
}

fn main() {
    let stdin = io::stdin();
    let stdout = io::stdout();
    let mut out = io::BufWriter::new(stdout.lock());
    let mut lines = stdin.lock().lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut max_heap: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        let x: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        if x == 0 {
            let max_val = pop_max(&mut max_heap);
            writeln!(out, "{}", max_val);
        } else {
            insert_int(&mut max_heap, x);
        }
    }
}