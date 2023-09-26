#[macro_use]
extern crate lazy_static;

use itertools::Itertools;
use std::borrow::Cow;
use std::collections::HashMap;
use std::iter::once;

lazy_static! {
    static ref ADJACENT: HashMap<char, Cow<'static, [char]>> = {
        let mut m = HashMap::with_capacity(10);
        m.insert('1', Cow::from(&['2', '4'][..]));
        m.insert('2', Cow::from(&['1', '5', '3'][..]));
        m.insert('3', Cow::from(&['2', '6'][..]));
        m.insert('4', Cow::from(&['1', '7', '5'][..]));
        m.insert('5', Cow::from(&['2', '4', '6', '8'][..]));
        m.insert('6', Cow::from(&['3', '5', '9'][..]));
        m.insert('7', Cow::from(&['4', '8'][..]));
        m.insert('8', Cow::from(&['5', '7', '9', '0'][..]));
        m.insert('9', Cow::from(&['6', '8'][..]));
        m.insert('0', Cow::from(&['8'][..]));
        m
    };
}

fn get_pins(observed: &str) -> Vec<String> {
    observed
        .chars()
        .into_iter()
        .filter_map(|c| {
            ADJACENT
                .get(&c)
                .map(|v| v.into_iter().map(|a| *a).chain(once(c)))
        })
        .multi_cartesian_product()
        .map(|v| v.iter().collect::<String>())
        .collect_vec()
}

#[cfg(test)]
mod tests {
    use super::get_pins;
    use itertools::Itertools;

    #[test]
    fn sample_tests() {
        assert_eq!(
            get_pins("8").iter().sorted().collect::<Vec<&String>>(),
            vec!["0", "5", "7", "8", "9"]
        );
        assert_eq!(
            get_pins("11").iter().sorted().collect::<Vec<&String>>(),
            vec!["11", "12", "14", "21", "22", "24", "41", "42", "44"]
        );
        assert_eq!(
            get_pins("369").iter().sorted().collect::<Vec<&String>>(),
            vec![
                "236", "238", "239", "256", "258", "259", "266", "268", "269", "296", "298", "299",
                "336", "338", "339", "356", "358", "359", "366", "368", "369", "396", "398", "399",
                "636", "638", "639", "656", "658", "659", "666", "668", "669", "696", "698", "699"
            ]
        );
    }
}
