fn likes(names: &[&str]) -> String {
    match names {
        [] => "no one likes this".into(),
        [name] => format!("{name} likes this"),
        [first, second] => format!("{first} and {second} like this"),
        [first, second, third] => format!("{first}, {second} and {third} like this"),
        [first, second, rest @ ..] => {
            let others_count = rest.len();
            format!("{first}, {second} and {others_count} others like this")
        }
    }
}
