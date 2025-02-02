fn leaders_in_array(v: &[i32]) -> Vec<i32> {
    if v.is_empty() {
        return Vec::<i32>::new();
    }
    let mut result: Vec<i32> = Vec::new();
    let mut leader: i32 = v[v.len() - 1] - 1;
    for &el in v.iter().rev() {
        if el > leader {
            result.push(el);
            leader = el;
        }
    }
    result
}

fn main() {
    leaders_in_array(&[]);
}
