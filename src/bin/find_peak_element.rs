use competitive_programming_and_contest_24::lib::lib_bs::binary_search_range;

fn find_peak_element(v: &[i32]) -> usize {
    let p = |middle: usize| -> bool {
        if middle != v.len() - 1 {
            return v[middle] < v[middle + 1];
        }
        true
    };
    let result: Option<usize> = binary_search_range(0, v.len(), p);
    match result {
        None => 0,
        Some(res) => {
            if res == v.len() - 1 {
                return res;
            }
            res + 1
        }
    }
}

fn main() {
    println!("{}", find_peak_element(&[7, 6, 5, 4, 3, 2, 1]));
}
