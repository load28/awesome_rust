pub fn brrow(str: &String) -> &str {
    let bytes = str.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[0..i];
        }
    }

    &str[..]
}
