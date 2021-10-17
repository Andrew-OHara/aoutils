pub fn ensure_newline(line: &str) -> String {
    if let Some(c) = line.chars().last() {
        if c != '\n' {
            return String::from(format!("{}{}", line, '\n'));
        }
    }

    return line.to_string();
}

/// True if the string is made up solely of alphabetic characters
///
/// # Arguments
///
/// * `s` - the &str to check
///
/// # Examples
///
/// ```    
/// use aoutils;
/// let result = aoutils::is_alphabetic("alpha");
/// assert_eq!(result, true);
/// ```
pub fn is_alphabetic(s : &str) -> bool {
    for c in s.chars() {
        if !c.is_alphabetic() {            
            return false
        }
    }

    true
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
