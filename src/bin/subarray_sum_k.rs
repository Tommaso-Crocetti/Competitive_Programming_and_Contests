use std::collections::HashMap;

fn subarray_sum_k(v: &[i32], k: i32) -> u32 {
    let mut prefix = vec![0; v.len()];
    prefix[0] = v[0];
    let mut result: u32 = if prefix[0] == k { 1 } else { 0 };
    for i in 1..v.len() {
        prefix[i] = v[i] + prefix[i - 1];
        if prefix[i] == k {
            result += 1;
        }
    }
    let mut map = HashMap::new();
    for value in prefix {
        result += map.get(&(value - k)).copied().unwrap_or(0);
        *map.entry(value).or_insert(0) += 1;
    }
    result
}

fn main() {
    println!("{}", subarray_sum_k(&[1, 2, 3, 4, 5], 9));
}
