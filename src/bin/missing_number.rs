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

#[cfg(test)]
mod missing_number_tests {
    use super::*;

    #[test]
    fn test2() {
        assert_eq!(3, missing_number(&[1, 2, 4]));
        assert_eq!(5, missing_number(&[3, 2, 1, 4, 6]));
    }

    #[test]
    // caso limite
    fn test1() {
        assert_eq!(1, missing_number(&[]));
    }
}
