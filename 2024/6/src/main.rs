// Write a function that returns the reference to the longer string
// without any new allocations
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

fn main() {
    let s1 = "hello";
    let s2 = "world";
    let longer = longer_wish(s1, s2);
    println!("{:?}", longer);
}