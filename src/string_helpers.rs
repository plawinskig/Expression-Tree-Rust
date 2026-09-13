
pub fn find_text(text: &str) -> Option<usize>
{
    for (i, chr) in text.chars().enumerate()
    {
        if ! chr.is_whitespace()
        {
            return Some(i);
        }
    }

    return None;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_find_text() {
        assert_eq!(find_text("     56789"), Some(5));
        assert_eq!(find_text("0123456789"), Some(0));
        assert_eq!(find_text("          "), None);
    }
}