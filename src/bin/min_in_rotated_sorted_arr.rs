use competitive_programming_and_contest_24::lib::lib_bs::binary_search_range;

fn min_in_rotated_sorted_arr(v: &[i32]) -> usize {
    let p = |middle: usize| -> bool { v[middle] >= v[0] };
    let result: Option<usize> = binary_search_range(0, v.len(), p);
    result.unwrap() + 1
}

fn main() {
    println!("{}", min_in_rotated_sorted_arr(&[4, 0, 1, 2]));
}
