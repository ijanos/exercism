pub fn abbreviate(text: &str) -> String {
    let mut acronym = text.chars().nth(0).unwrap().to_string();
    let mut prev = text.chars().nth(0).unwrap();
    for c in text.chars().skip(1) {
        if c.is_uppercase() && !prev.is_uppercase() ||
           prev.is_whitespace() ||
           !prev.is_alphanumeric()
           && !c.is_whitespace() {
               acronym.push(c);
        }
        prev = c;
    }
    acronym.to_uppercase()
}