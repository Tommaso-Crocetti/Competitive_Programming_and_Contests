fn check_all_covered_in_range(v: &[(i32, i32)], left: i32, right: i32) -> bool {
    let mut i: i32 = left;
    while i <= right {
        let mut result: bool = false;
        for &(start, end) in v {
            if i >= start && i <= end {
                result = true;
                break;
            }
        }
        if !result {
            return false;
        }
        i += 1;
    }
    true
}

fn main() {
    println!(
        "{}",
        check_all_covered_in_range(&[(1, 2), (3, 4), (4, 5)], 2, 5)
    );
}
