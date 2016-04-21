pub fn number(number: &str) -> Option<String> {
    let numbers: String = number.chars().filter(|&c| c.is_numeric()).collect();
    match numbers {
        _ if numbers.len() == 10 => Some(numbers),
        _ if numbers.starts_with("1") && numbers.len() == 11 => {
            Some(numbers.chars().skip(1).collect())
        }
        _ => None,
    }
}

pub fn area_code(num: &str) -> Option<String> {
    if let Some(numbers) = number(num) {
        Some(numbers.chars().take(3).collect())
    } else {
        None
    }
}

pub fn pretty_print(num: &str) -> String {
    if let Some(numbers) = number(num) {
        let (area, end) = numbers.split_at(3);
        let (part1, part2) = end.split_at(3);
        format!("({}) {}-{}", area, part1, part2)
    } else {
        "invalid".into()
    }
}
