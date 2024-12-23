pub(crate) fn case_insensitive_match(left: &str, right: &str) -> bool {
    let left = left.chars().map(|c| c.to_lowercase()).flatten();
    let right = right.chars().map(|c| c.to_lowercase()).flatten();

    left.eq(right)
}
