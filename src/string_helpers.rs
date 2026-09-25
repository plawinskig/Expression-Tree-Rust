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
    let mut result: Vec<&str> = Vec::new();

    let mut next_regex_index = input.find(separator);
    let regex_length = separator.len();
    let mut offset = 0;

    while let Some(index) = next_regex_index {
        if index > offset {
            let next_split = index - offset;
            let element = cut_white_beginning(&input[offset..next_split]);

            if ! element.is_empty() {
                result.push(element);
            }
        }

        offset = index + regex_length;
        next_regex_index = input[offset..]
            .find(separator)
            .map(|new_index| new_index + offset);
    }

    if offset < input.len() {
        let element = cut_white_beginning(&input[offset..]);

        if ! element.is_empty() {
            result.push(element);
        }
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
}
