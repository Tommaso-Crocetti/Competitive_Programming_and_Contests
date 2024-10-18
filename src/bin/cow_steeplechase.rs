use std::collections::BTreeSet;

fn intersects(segment: (i32, i32, i32, i32), other: (i32, i32, i32, i32)) -> bool {
    let (s0, s1, s2, s3) = (
        segment.0 as f64,
        segment.1 as f64,
        segment.2 as f64,
        segment.3 as f64,
    );
    let (o0, o1, o2, o3) = (
        other.0 as f64,
        other.1 as f64,
        other.2 as f64,
        other.3 as f64,
    );
    let num_1 = s3 - s1;
    let den_1 = s2 - s0;
    let m_1: Option<f64> = if den_1 == 0.0 {
        None
    } else {
        Some(num_1 / den_1)
    };
    let num_2 = o3 - o1;
    let den_2 = o2 - o0;
    let m_2: Option<f64> = if den_2 == 0.0 {
        None
    } else {
        Some(num_2 / den_2)
    };
    match (m_1, m_2) {
        (Some(m1), Some(m2)) => {
            let q1 = s1 - (m1 * s0);
            let q2 = o1 - (m2 * o0);
            let x_res = (q2 - q1) / (m1 - m2);
            (s0 <= x_res && x_res <= s2 && o0 <= x_res && x_res <= o2)
                || (s2 <= x_res && x_res <= s0 && o2 <= x_res && x_res <= o0)
        }
        (None, Some(m2)) => {
            let x_res = s0;
            if o0 <= x_res && x_res <= o2 {
                let q2 = o1 - (m2 * o0);
                let y_res = m2 * x_res + q2;
                (s1 <= y_res && y_res <= s3) || (s3 <= y_res && y_res <= s1)
            } else {
                false
            }
        }
        (Some(m1), None) => {
            let x_res = o0;
            if s0 <= x_res && x_res <= s2 {
                let q1 = s1 - (m1 * s0);
                let y_res = m1 * x_res + q1;
                (o1 <= y_res && y_res <= o3) || (o3 <= y_res && y_res <= o1)
            } else {
                false
            }
        }
        (None, None) => s0 == o0,
    }
}

fn cow_steeplechase(v: &[(i32, i32, i32, i32)]) -> usize {
    let mut events = Vec::with_capacity(v.len() * 2);
    let mut active = BTreeSet::<usize>::new();
    let mut remove = None;
    for (i, (x_start, y_start, x_end, y_end)) in v.iter().enumerate() {
        events.push((i, (x_start, y_start), true));
        events.push((i, (x_end, y_end), false));
    }
    events.sort_by(
        |a: &(usize, (&i32, &i32), bool), b: &(usize, (&i32, &i32), bool)| a.1 .0.cmp(b.1 .0),
    );
    for (index, _, start) in events {
        let current = v[index];
        if start {
            for active_idx in &active {
                let active_seg = v[*active_idx];
                if intersects(current, active_seg) {
                    remove = Some(index);
                    break;
                }
            }
            active.insert(index);
        } else {
            active.remove(&index);
        }
        if let Some(result) = remove {
            return result;
        }
    }
    v.len()
}

fn main() {
    println!(
        "{}",
        cow_steeplechase(&[(2, 1, 6, 1), (4, 0, 1, 5), (5, 6, 5, 5), (2, 7, 1, 3),])
    );
}
