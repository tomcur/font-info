pub(crate) fn case_insensitive_match(left: &str, right: &str) -> bool {
    let mut left = left.chars();
    let mut right = right.chars();

    loop {
        match (left.next(), right.next()) {
            (None, None) => return true,
            (left, right) => {
                if left != right {
                    return false;
                }
            }
        }
    }
}
