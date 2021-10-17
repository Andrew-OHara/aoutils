/// Returns a String that ends with a '\n' newline character
///
/// # Arguments
///
/// * `line` - the &str to ensure ends in a newline
///
/// # Examples
///
/// ```    
/// use aoutils;
///
/// let result = aoutils::ensure_newline("alpha");
/// assert!(result.ends_with("\n"));
/// ```
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
    use super::*;
    
    #[test]
    fn ensure_newline_without_newline() {        
        let result = ensure_newline("test string");
        assert!(result.ends_with("\n"));
    }
    
    #[test]
    fn ensure_newline_with_newline() { 
        let test_string = "test_string\n";       
        
        let result = ensure_newline(test_string);
        assert!(result.ends_with("\n"));
        assert!(result.matches("\n").count() == 1);
    }

    #[test]
    fn is_alphabetic_with_alphabetic() {
        assert_eq!(is_alphabetic("alpha"), true);
    }

    #[test]
    fn is_alphabetic_with_numeric() {        
        assert_eq!(is_alphabetic("alpha1"), false);
    }
}