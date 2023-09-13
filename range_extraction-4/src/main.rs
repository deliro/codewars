mod solution {
    use itertools::Itertools;

    #[derive(Copy, Clone)]
    enum Kind<T> {
        Range(T, T),
        Single(T),
    }

    pub fn range_extraction(a: &[i32]) -> String {
        a.into_iter()
            .map(|v| Kind::Single(*v))
            .coalesce(|x, y| match (x, y) {
                (Kind::Range(q, p), Kind::Single(w)) => {
                    if p + 1 == w {
                        Ok(Kind::Range(q, w))
                    } else {
                        Err((x, y))
                    }
                }
                (Kind::Single(q), Kind::Single(p)) => {
                    if q + 1 == p {
                        Ok(Kind::Range(q, p))
                    } else {
                        Err((x, y))
                    }
                }
                _ => panic!("unbelievable"),
            })
            .map(|v| match v {
                Kind::Range(q, p) => {
                    if q + 1 != p {
                        format!("{q}-{p}")
                    } else {
                        format!("{q},{p}") // It is not considered a range unless it spans at least 3 numbers
                    }
                }
                Kind::Single(q) => q.to_string(),
            })
            .join(",")
    }
}
