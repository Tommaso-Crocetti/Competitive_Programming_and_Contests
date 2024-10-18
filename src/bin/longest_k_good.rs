use std::collections::HashMap;

fn longest_k_good(v: &[i32], k: usize) -> (usize, usize) {
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut map = HashMap::new();
    let mut max_len: usize = 0;
    let mut best_l: usize = 0;
    let mut best_r: usize = 0;

    while right < v.len() {
        *map.entry(v[right]).or_insert(0) += 1;
        while map.len() > k {
            if let Some(count) = map.get_mut(&v[left]) {
                *count -= 1;
                if *count == 0 {
                    map.remove(&v[left]);
                }
            }
            left += 1;
        }
        if right - left + 1 > max_len {
            max_len = right - left + 1;
            best_l = left;
            best_r = right;
        }
        right += 1;
    }
    (best_l, best_r)
}

fn main() {
    let (l, r) = longest_k_good(&[1, 1, 2, 3, 3, 4, 4, 1, 3], 3);
    println!("{}, {}", l, r);
}
