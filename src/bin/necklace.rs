fn same_necklace(first: &str, second: &str) -> bool {
    let mut twice = String::new();
    if first.len() == second.len() {
        twice.push_str(first);
        twice.push_str(first);
        twice.contains(second)
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test() {
        assert_eq!(same_necklace("nicole", "icolen"), true);
        assert_eq!(same_necklace("nicole", "lenico"), true);
        assert_eq!(same_necklace("nicole", "coneli"), false);
        assert_eq!(same_necklace("aabaaaaabaab", "aabaabaabaaa"), true);
        assert_eq!(same_necklace("abc", "cba"), false);
        assert_eq!(same_necklace("xxyyy", "xxxyy"), false);
        assert_eq!(same_necklace("xyxxz", "xxyxz"), false);
        assert_eq!(same_necklace("x", "x"), true);
        assert_eq!(same_necklace("x", "xx"), false);
        assert_eq!(same_necklace("x", ""), false);
        assert_eq!(same_necklace("", ""), true);
    }
}
