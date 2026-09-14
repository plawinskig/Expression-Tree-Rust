pub fn find_text(text: &str) -> Option<usize> {
    for (i, chr) in text.chars().enumerate() {
        if !chr.is_whitespace() {
            return Some(i);
        }
    }

    return None;
}

fn crop_letters(s: &str, pos: usize) -> &str {
    match s.char_indices().skip(pos).next() {
        Some((pos, _)) => &s[pos..],
        None => "",
    }
}

pub fn cut_white_beginning(text: &str) -> &str {
    let pos = find_text(text);

    match pos {
        Some(pos) if pos > 0 => crop_letters(text, pos),
        None if !text.is_empty() => "",
        _ => text,
    }
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
