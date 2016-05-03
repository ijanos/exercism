#[derive(Debug,PartialEq)]
pub enum Comparison {
    Equal,
    Unequal,
    Sublist,
    Superlist,
}

pub fn sublist<T: PartialEq>(l1: &[T], l2: &[T]) -> Comparison {
    let contains = |l1: &[T], l2: &[T]| l1.windows(l2.len()).any(|win| win == l2);
    match (&l1, &l2) {
        _ if &l1 == &l2 => Comparison::Equal,
        _ if l1.is_empty() => Comparison::Sublist,
        _ if l2.is_empty() => Comparison::Superlist,
        _ if contains(l2, l1) => Comparison::Sublist,
        _ if contains(l1, l2) => Comparison::Superlist,
        _ => Comparison::Unequal,
    }
}
