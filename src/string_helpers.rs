
fn find_text(text: &str) -> Option<usize>
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