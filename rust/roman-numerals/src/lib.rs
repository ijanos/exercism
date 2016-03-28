pub struct Roman;

fn romanize(low: char, mid: char, high: char, number: u32) -> String {
    match number {
        n @ 1 ... 3 => vec![low; n as usize].into_iter().collect::<String>(),
        4 => vec![low, mid].into_iter().collect::<String>(),
        n @ 5 ... 8 => {
            let mut roman = vec![mid];
            roman.append(&mut vec![low; n as usize - 5]);
            roman.into_iter().collect()
        }
        9 => vec![low, high].into_iter().collect::<String>(),
        _ => "".to_owned(),
    }

}

fn convert((position, digit): (usize, char)) -> String {
    let digit = digit.to_digit(10).unwrap();
    match position {
        0 => romanize('I', 'V', 'X', digit),
        1 => romanize('X', 'L', 'C', digit),
        2 => romanize('C', 'D', 'M', digit),
        3 => romanize('M', '_', '_', digit),
        _ => panic!(),
    }
}

impl Roman {
    pub fn from(number: u32) -> String {
        number.to_string()
              .chars()
              .rev()
              .enumerate()
              .collect::<Vec<_>>()
              .into_iter()
              .rev()
              .map(convert)
              .collect::<String>()
    }
}
