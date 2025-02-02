fn missing_number(v: &[i32]) -> i32 {
    let max: i32 = (v.len() + 1).try_into().unwrap();
    let mut result = (max * (max + 1)) / 2;
    for &el in v {
        result -= el;
    }
    result
}

fn main() {
    missing_number(&[]);
}
