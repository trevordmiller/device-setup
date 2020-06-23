pub fn underline(content: &str, symbol: &str) -> String {
    content.chars().map(|_| symbol).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_underlines_with_equals() {
        assert_eq!(underline("Some line", "="), "=========");
    }

    #[test]
    fn it_underlines_with_dashes() {
        assert_eq!(underline("Another line", "-"), "------------");
    }
}
