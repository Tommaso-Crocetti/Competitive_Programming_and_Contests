fn trapping_rain_water(v: &[i32]) -> i32 {
    if v.is_empty() {
        return 0;
    }
    let mut result: i32 = 0;
    let mut border: i32 = v[0];
    let mut iter: i32 = 0;
    let mut counter: i32 = 0;
    for &el in v.iter() {
        if el >= border {
            result += iter * border - counter;
            border = el;
            iter = 0;
            counter = 0;
        } else {
            iter += 1;
            counter += el;
        }
    }
    border = v[v.len() - 1];
    iter = 0;
    counter = 0;
    for &el in v.iter().rev() {
        if el >= border {
            result += iter * border - counter;
            border = el;
            iter = 0;
            counter = 0;
        } else {
            iter += 1;
            counter += el;
        }
    }
    result
}

fn main() {
    trapping_rain_water(&[]);
}

#[cfg(test)]
mod trapping_rain_water_tests {
    use super::*;

    #[test]
    fn test2() {
        assert_eq!(0, trapping_rain_water(&[3, 2, 1]));
        assert_eq!(0, trapping_rain_water(&[1, 2, 3]));
        assert_eq!(10, trapping_rain_water(&[3, 0, 0, 2, 0, 4]));
    }

    #[test]
    // casi limite
    fn test1() {
        assert_eq!(0, trapping_rain_water(&[]));
    }
}
