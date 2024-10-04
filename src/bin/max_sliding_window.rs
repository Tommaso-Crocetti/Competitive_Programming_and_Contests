use std::collections::VecDeque;

fn max_sliding_window(v: &[i32], k: usize) -> Vec<i32> {
    let n: usize = v.len();
    if k == 0 || k > n {
        return Vec::<i32>::new();
    }
    let mut result: Vec<i32> = Vec::with_capacity(n - k + 1);
    let mut q: VecDeque<usize> = VecDeque::new();
    for i in 0..k {
        while !(q.is_empty()) && v[i] > v[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
    }
    result.push(v[*q.front().unwrap()]);
    for i in k..n {
        while !(q.is_empty()) && *q.front().unwrap() <= i - k {
            q.pop_front();
        }
        while !(q.is_empty()) && v[i] > v[*q.back().unwrap()] {
            q.pop_back();
        }
        q.push_back(i);
        result.push(v[*q.front().unwrap()]);
    }
    result
}

fn main() {
    max_sliding_window(&[], 0);
}
