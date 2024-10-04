use competitive_programming_and_contest_24::lib::lib_bs::binary_search_range;

fn first_and_last_occurence(v: &[i32], target: i32) -> (Option<usize>, Option<usize>) {
    let p1 = |middle: usize| -> bool { v[middle] < target };
    let p2 = |middle: usize| -> bool { v[middle] <= target };
    let right: Option<usize> = binary_search_range(0, v.len(), p2);
    let left: Option<usize> = binary_search_range(0, v.len(), p1);
    if right.is_none() || left == right {
        (None, None)
    } else if left.is_none() {
        return (Some(0), right);
    } else {
        return (Some(left.unwrap() + 1), right);
    }
}

fn main() {
    println!(
        "{:?}",
        first_and_last_occurence(&[1, 2, 2, 4, 4, 5, 5, 5, 6, 7, 8, 9], 5)
    );
}

#[cfg(test)]
mod first_and_last_occurence_tests {
    use super::*;

    #[test]
    fn test2() {
        assert_eq!(
            (Some(3), Some(5)),
            first_and_last_occurence(&[5, 6, 6, 8, 8, 8, 10], 8)
        );
        assert_eq!(
            (Some(0), Some(0)),
            first_and_last_occurence(&[5, 7, 7, 8, 8, 10], 5)
        );
    }

    #[test]
    fn test1() {
        assert_eq!(
            (None, None),
            first_and_last_occurence(&[5, 6, 6, 8, 8, 8, 10], 7)
        );
        assert_eq!((None, None), first_and_last_occurence(&[], 1));
    }
}
