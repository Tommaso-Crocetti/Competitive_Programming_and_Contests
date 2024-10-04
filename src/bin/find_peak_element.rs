use competitive_programming_and_contest_24::lib::lib_bs::binary_search_range;

fn find_peak_element(v: &[i32]) -> usize {
    let p = |middle: usize| -> bool { v[middle] < v[middle + 1] };
    let result: Option<usize> = binary_search_range(0, v.len(), p);
    result.unwrap() + 1
}

fn main() {
    println!("{}", find_peak_element(&[1, 2, 1, 3, 5, 6, 4]));
}
