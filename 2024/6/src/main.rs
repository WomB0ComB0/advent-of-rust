//! This module provides string comparison functionality based on length.

/// Compares two string slices and returns a reference to the longer one.
///
/// # Arguments
///
/// * `s1` - First string slice to compare
/// * `s2` - Second string slice to compare
///
/// # Returns
///
/// * `Some(&str)` - Reference to the longer string if one exists
/// * `None` - If both strings are equal in length after trimming
///
/// # Examples
///
/// ```
/// let s1 = "hello  ";
/// let s2 = "world";
/// assert_eq!(longer_wish(s1, s2), Some("hello  "));
/// ```
///
/// # Notes
///
/// - The function trims whitespace before comparing lengths
/// - Returns original (untrimmed) string reference
/// - No new string allocations are made
/// - Uses Unicode scalar values for length comparison via chars().count()
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1_trimmed = s1.trim();
    let s2_trimmed = s2.trim();

    let s1_chars = s1_trimmed.chars().count();
    let s2_chars = s2_trimmed.chars().count();

    if s1_chars > s2_chars {
        Some(s1)
    } else if s2_chars > s1_chars {
        Some(s2)
    } else {
        None
    }
}

/// Example usage of the longer_wish function
fn main() {
    let s1 = "hello";
    let s2 = "world";
    let longer = longer_wish(s1, s2);
    println!("{:?}", longer);
}
