pub fn reply(input: &str) -> &str {
    match input {
        "" => "Fine. Be that way!",
        _ if input.chars().last().unwrap() == '?' => "Sure.",
        _ if input == input.to_uppercase() => "Whoa, chill out!",
        _ =>  "Whatever."
    }
}
