fn move_zeros(arr: &[u8]) -> Vec<u8> {
    let mut no_zeros: Vec<_> = arr
        .into_iter()
        .filter_map(|&x| if x != 0 { Some(x) } else { None })
        .collect();
    (0..arr.len() - no_zeros.len()).for_each(|_| no_zeros.push(0));
    no_zeros
}
