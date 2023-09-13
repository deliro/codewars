mod preloaded;
use itertools::Itertools;
use preloaded::MORSE_CODE;

// MORSE_CODE is `HashMap<String, String>`. e.g. ".-" -> "A".

pub fn decode_bits(encoded: &str) -> String {
    let encoded = encoded.trim_matches('0');
    if encoded.len() == 0 {
        return "".to_string();
    }
    let groups = {
        let mut tmp = encoded
            .chars()
            .group_by(|&v| v)
            .into_iter()
            .map(|(ch, g)| (ch, g.count()))
            .collect_vec();
        let freq = tmp
            .iter()
            .min_by(|(_, x), (_, y)| x.cmp(y))
            .map(|(_, cnt)| *cnt)
            .unwrap_or(1);
        tmp.iter_mut().for_each(|(_, cnt)| *cnt /= freq);
        tmp
    };

    let mut word = String::with_capacity(groups.len() / 2);
    for (ch, cnt) in groups {
        match (ch, cnt) {
            ('1', 1) => word.push('.'),
            ('1', 3) => word.push('-'),
            ('0', 3) => word.push(' '),
            ('0', 7) => word.push_str("   "),
            _ => {}
        }
    }
    word
}

fn decode_morse(encoded: &str) -> String {
    encoded
        .trim()
        .split("   ")
        .into_iter()
        .map(|w| w.split(" "))
        .map(|chars| chars.filter_map(|ch| MORSE_CODE.get(ch)).join(""))
        .join(" ")
}
