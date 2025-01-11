
pub fn pair_merge<L, R>(first: Option<L>, second: Option<R>) -> Option<(L, R)> {
    match (first, second) {
        (Some(first_value), Some(second_value)) => Some((first_value, second_value)),
        _ => None
    }
}
