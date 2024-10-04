fn leaders_in_array(v: &[i32]) -> Vec<i32> {
    if v.is_empty() {
        return Vec::<i32>::new();
    }
    let mut result: Vec<i32> = Vec::new();
    let mut leader: i32 = v[v.len() - 1] - 1;
    for &el in v.iter().rev() {
        if el > leader {
            result.push(el);
            leader = el;
        }
    }
    result
}

fn main() {
    leaders_in_array(&[]);
}

#[cfg(test)]
mod leaders_in_array_tests {
    use super::*;

    #[test]
    fn test2() {
        assert_eq!([1, 2, 3].to_vec(), leaders_in_array(&[3, 2, 1]));
    }

    #[test]
    // casi limite
    fn test1() {
        assert_eq!(Vec::<i32>::new(), leaders_in_array(&[]));
        assert_eq!([5].to_vec(), leaders_in_array(&[1, 2, 3, 4, 5]));
    }
}
