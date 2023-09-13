use std::cmp::Ordering;
use std::iter::repeat;

use itertools::Itertools;

fn mix(s1: &str, s2: &str) -> String {
    let s1_counts = s1.chars().filter(|v| ('a'..='z').contains(v)).counts();
    let s2_counts = s2.chars().filter(|v| ('a'..='z').contains(v)).counts();
    s1_counts
        .iter()
        .zip(repeat(1))
        .chain(s2_counts.iter().zip(repeat(2)))
        .map(|((ch, cnt), n)| (*ch, *cnt, n))
        .sorted_by(|(ch1, a, _), (ch2, b, _)| match b.cmp(a) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => ch1.cmp(ch2),
        })
        .take_while(|(_, a, _)| *a > 1)
        .unique_by(|(ch, _, _)| *ch)
        .map(|(ch, cnt, n)| {
            let other_cnt = *(if n == 1 {
                s2_counts.get(&ch).unwrap_or(&0)
            } else {
                s1_counts.get(&ch).unwrap_or(&0)
            });

            let (max_cnt, max_n) = if cnt > other_cnt {
                (cnt, if n == 1 { '1' } else { '2' })
            } else if cnt < other_cnt {
                (other_cnt, if n == 1 { '2' } else { '1' })
            } else {
                (cnt, '=')
            };

            (max_n, ch.to_string().repeat(max_cnt))
        })
        .sorted_by(|(a, rep1), (b, rep2)| match rep2.len().cmp(&rep1.len()) {
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
            Ordering::Equal => a.cmp(b),
        })
        .map(|(n, r)| format!("{n}:{r}"))
        .join("/")
}
