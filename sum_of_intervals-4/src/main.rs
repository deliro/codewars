use itertools::Itertools;
use std::cmp::{max, min};

fn intersection<T: Ord + Copy>(r1: (T, T), r2: (T, T)) -> Option<(T, T)> {
    let l = max(&r1.0, &r2.0);
    let r = min(&r1.1, &r2.1);
    if l > r {
        None
    } else {
        Some((*l, *r))
    }
}

fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    intervals
        .into_iter()
        .sorted_by(|(x1, _), (x2, _)| x1.cmp(x2))
        .map(|(x, y)| (*x, *y))
        .coalesce(|x, y| match intersection(x, y) {
            Some(_) => Ok((x.0, max(x.1, y.1))),
            None => Err((x, y)),
        })
        .map(|(l, r)| r - l)
        .sum()
}
