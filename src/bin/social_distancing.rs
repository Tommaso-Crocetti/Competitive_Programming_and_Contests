use competitive_programming_and_contest_24::lib::lib_bs::binary_search_range;

fn social_distancing(v: &mut [(usize, usize)], c: usize) -> Option<usize> {
    v.sort();
    let length = v
        .iter()
        .fold(0, |acc, interval| acc + (interval.1 - interval.0));
    let p = |middle: usize| -> bool {
        let mut border = v[0].0;
        let mut counter = 1;
        for &interval in v.iter() {
            while interval.0.max(border + middle) <= interval.1 {
                counter += 1;
                border = interval.0.max(border + middle);
            }
        }
        counter >= c
    };
    binary_search_range(1, length + 1, p)
}

fn main() {
    social_distancing(&mut [], 0);
}
