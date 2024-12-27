

fn captures_into_vec(captures: regex::Captures) -> Vec<String> {
    captures.iter()
        .map_while(|group|group)
        .map(|group|group.as_str().to_string())
        .collect()
}

/// Given a regex `re` and a `haystack` it returns an option to a vector of strings,
/// representing the captured groups. If the returned option is `None`, that means
/// pattern described in `re` was not found in the `haystack`. Otherwise, it returns
/// `Some(v)` where `v[0]` is the whole match, and subsequent items - if any- `v[1..]`
/// are captured groups. Be mindful though, e.g. for a regex pattern `(\w)(\d)?(\w)`
/// depending whether the middle group is matched or not it can return variable number
/// of captured groups.
pub fn get_captures(re: &regex::Regex, haystack: &str) -> Option<Vec<String>> {
    re.captures(haystack).map(captures_into_vec)
}