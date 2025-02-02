fn check_all_covered_in_range(v: &mut [(i32, i32)], left: i32, right: i32) -> bool {
    v.sort_by(|a, b| a.0.cmp(&b.0));
    let mut i: i32 = left;
    for &mut (start, end) in v {
        if start > i {
            return false;
        }
        i = i.max(end + 1);
        if i >= right {
            return true;
        }
    }
    false
}

fn main() {
    println!(
        "{}",
        check_all_covered_in_range(
            &mut [
                (25, 42),
                (7, 14),
                (2, 32),
                (25, 28),
                (39, 49),
                (29, 45),
                (18, 47)
            ],
            15,
            38
        )
    );
}
