fn title(t: &str) -> String {
    t.chars()
        .enumerate()
        .map(|(i, x)| {
            if i == 0 {
                x.to_ascii_uppercase()
            } else {
                x.to_ascii_lowercase()
            }
        })
        .collect()
}

fn save_case(t: &str) -> String {
    let first = t.chars().next().unwrap();
    if first.is_lowercase() {
        t.to_lowercase()
    } else {
        title(t)
    }
}
fn to_camel_case(text: &str) -> String {
    if text.len() == 0 {
        return "".to_string();
    }
    text.split(|v: char| !v.is_ascii_alphabetic())
        .filter(|w| w.len() > 0)
        .enumerate()
        .map(|(i, w)| if i == 0 { save_case(w) } else { title(w) })
        .collect()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::to_camel_case;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn dotest(s: &str, expected: &str) {
        assert_eq!(to_camel_case(s), expected, "{ERR_MSG} with text = \"{s}\"")
    }

    #[test]
    fn fixed_tests() {
        dotest("", "");
        dotest("the_stealth_warrior", "theStealthWarrior");
        dotest("The-Stealth-Warrior", "TheStealthWarrior");
        dotest("A-B-C", "ABC");
    }
}
