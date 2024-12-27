use std::fmt::Display;

pub fn vector_display<T>(v: &Vec<T>, separator: &str) -> String
where T: Display {
    v.iter()
        .map(|item|item.to_string())
        .collect::<Vec<_>>()
        .join(separator)
}