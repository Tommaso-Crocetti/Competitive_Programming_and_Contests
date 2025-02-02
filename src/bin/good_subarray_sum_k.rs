use std::collections::HashMap;

fn good_subarray_sum_k(v: &[i32], k: i32) -> bool {
    let mut prefix = vec![0; v.len()];
    prefix[0] = v[0];
    for i in 1..v.len() {
        prefix[i] = v[i] + prefix[i - 1];
        if prefix[i] % k == 0 {
            return true;
        }
    }
    let mut map = HashMap::new();
    for (i, value) in prefix.iter().enumerate() {
        if let Some(index) = map.get(&(value % k)) {
            if i - index > 1 {
                return true;
            }
        }
        let _ = *map.entry(value % k).or_insert(i);
    }
    false
}

fn main() {
    println!("{}", good_subarray_sum_k(&[1, 2, 3, 4, 5], 9));
}
