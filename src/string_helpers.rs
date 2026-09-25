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

pub fn split<'a>(input: &'a str, separator: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    
    let mut tail = input; 
    let sep_len = separator.len();

    while let Some(index) = tail.find(separator) {
        let chunk = &tail[..index]; 
        let element = cut_white_beginning(chunk);
        
        if !element.is_empty() {
            result.push(element);
        }

        tail = &tail[index + sep_len..];
    }

    let element = cut_white_beginning(tail);
    if !element.is_empty() {
        result.push(element);
    }

    result
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

    #[test]
    fn test_cut_white_beginning() {
        assert_eq!(cut_white_beginning("     56789"), "56789");
        assert_eq!(cut_white_beginning("0123456789"), "0123456789");
        assert_eq!(cut_white_beginning("01234     "), "01234     ");
    }

    #[test]
    fn test_split() {
        assert_eq!(split("No changes here.", ","), ["No changes here."]);
        assert_eq!(split("There is a   space between us.", " "), ["There", "is", "a", "space", "between", "us."]);
        let empty_res: Vec<&str> = Vec::new();
        assert_eq!(split("aaaaaa", "a"), empty_res);
    }
}
