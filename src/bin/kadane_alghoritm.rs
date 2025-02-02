fn kadane(v: &[i32]) -> i32 {
    let mut result: i32 = 0;
    let mut sum: i32 = 0;
    for &el in v {
        if sum > 0 {
            sum += el;
        } else {
            sum = el;
        }
        if sum > result {
            result = sum;
        }
    }
    result
}

fn main() {
    kadane(&[]);
}
