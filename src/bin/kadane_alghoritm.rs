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

#[cfg(test)]
mod kadane_tests {
    use super::*;

    #[test]
    fn test2() {
        assert_eq!(6, kadane(&[3, 2, 1]));
        assert_eq!(13, kadane(&[-1, 5, 8, -9, 4, 1]));
    }

    #[test]
    // casi limite
    fn test1() {
        assert_eq!(0, kadane(&[]));
        assert_eq!(0, kadane(&[-1, -2, -4]));
    }
}
