use std::collections::BTreeSet;

fn intersects(segment: (i32, i32, i32, i32), other: (i32, i32, i32, i32)) -> bool {
    fn orientation(px: i32, py: i32, qx: i32, qy: i32, rx: i32, ry: i32) -> i32 {
        (qy - py) * (rx - qx) - (qx - px) * (ry - qy)
    }

    fn on_segment(px: i32, py: i32, qx: i32, qy: i32, rx: i32, ry: i32) -> bool {
        (qx <= px.max(rx) && qx >= px.min(rx)) && (qy <= py.max(ry) && qy >= py.min(ry))
    }

    let o1 = orientation(segment.0, segment.1, segment.2, segment.3, other.0, other.1);
    let o2 = orientation(segment.0, segment.1, segment.2, segment.3, other.2, other.3);
    let o3 = orientation(other.0, other.1, other.2, other.3, segment.0, segment.1);
    let o4 = orientation(other.0, other.1, other.2, other.3, segment.2, segment.3);

    if o1 != o2 && o3 != o4 {
        return true;
    }

    (o1 == 0 && on_segment(segment.0, segment.1, other.0, other.1, segment.2, segment.3)) ||
    (o2 == 0 && on_segment(segment.0, segment.1, other.2, other.3, segment.2, segment.3)) ||
    (o3 == 0 && on_segment(other.0, other.1, segment.0, segment.1, other.2, other.3)) ||
    (o4 == 0 && on_segment(other.0, other.1, segment.2, segment.3, other.2, other.3))
}


fn cow_steeplechase(v: &[(i32, i32, i32, i32)]) -> usize {
    let mut events = Vec::with_capacity(v.len() * 2);
    let mut active = BTreeSet::<usize>::new();
    let mut remove = None;
    let mut i: usize = 0;
    for (x_start, y_start, x_end, y_end) in v {
        events.push((i, (x_start, y_start), true));
        events.push((i, (x_end, y_end), false));
        i += 1;
    }
    events.sort_by(|a: &(usize, (&i32, &i32), bool), b: &(usize, (&i32, &i32), bool)| {
        return a.1.0.cmp(&b.1.0);
    });
    for (index, _, start) in events {
        let current = v[index];
        if start {
            for active_idx in &active {
                let active_seg = v[*active_idx];
                if intersects(current, active_seg) {
                    remove = Some(index.max(*active_idx));
                    break;
                }
            }
            println!("insert");
            active.insert(index);
        } else {
            println!("rem");
            active.remove(&index);
        }
        if let Some(result) = remove {
            return result;
        }
    }
    return v.len();
}

fn main() {
    println!("{}", cow_steeplechase(&[(2,1,6,1), (4,0,1,5), (5,6,5,5), (2,7,1,3)]));
}