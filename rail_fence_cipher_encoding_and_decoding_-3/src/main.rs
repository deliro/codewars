use itertools::Itertools;

fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    assert!(num_rails >= 2);
    let mut strings = vec![String::with_capacity(text.len() / num_rails); num_rails];
    let it = ((0..num_rails).chain((0..num_rails).rev()))
        .cycle()
        .coalesce(|a, b| if a == b { Ok(a) } else { Err((a, b)) });
    text.chars().zip(it).for_each(|(c, idx)| {
        let s = &mut strings[idx];
        s.push(c)
    });

    strings.into_iter().collect()
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    encode_rail_fence_cipher(text, num_rails)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encoding() {
        assert_eq!(
            encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3),
            "WECRLTEERDSOEEFEAOCAIVDEN"
        );
        assert_eq!(
            encode_rail_fence_cipher("Hello, World!", 3),
            "Hoo!el,Wrdl l"
        );
    }

    #[test]
    fn decoding() {
        assert_eq!(
            decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3),
            "WEAREDISCOVEREDFLEEATONCE"
        );
        assert_eq!(
            decode_rail_fence_cipher("Hoo!el,Wrdl l", 3),
            "Hello, World!"
        );
    }
}

fn main() {}
// let period = (num_rails - 2) * (2) + 2;
//
//
// 0       4       8      12       L       T       E
//   1   3   5   7   9  11  13   F   E   A   O   C
//     2       6      10      14       E       N
//
//
// 0           6           12
//   1       5   7       11   13
//     2   4       8   10       14
//       3           9             15
//
// 0       8        16
//  1     7 9      15
//   2   6   10   14
//    3 5     11 13
//     4       12
//
// 6:
//
// 0         10
//  1       9  11
//   2     8    12
//    3   7      13
//     4 6        14
//      5          15

// 0 2 4 6
//  1 3 5 7

// 4:
// 2 -> pad, 3, 1, 3, 1
// 3 -> pad, 1, 3, 1, 3

// 5: period 8
// 0 -> 7, 0, 7, 0
// 1 -> 5, 1, 5, 1
// 2 -> 3, 3, 3, 3
// 3 -> 1, 5, 1, 5
// 4 -> 7, 0, 7, 0

// 6: period 10
// 0 -> 9, 0, 9, 0
// 1 -> 7, 1, 7, 1
// 2 -> 5, 3, 5, 3
// 3 -> 3, 5, 3, 5
// 4 -> 1, 7, 1, 7
