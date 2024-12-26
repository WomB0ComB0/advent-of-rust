use std::fmt;
// 1. Add the `is_wrapped` field to the gift structs
pub struct KidsGift {
    pub name: String,
    pub is_wrapped: bool,
}
pub struct ElvesGift {
    pub name: String,
    pub is_wrapped: bool,
}
pub struct ReindeerGift {
    pub name: String,
    pub is_wrapped: bool,
}
pub trait Gift {
    // 2. Finish the trait definition //
    fn wrap(&mut self);
} 
// 3. Update the function signature
pub fn prepare_gift<T: fmt::Display + Gift>(gift: &mut T) {
    println!("Preparing gift for {}", &gift);
    gift.wrap();
    println!("Gift wrapped for {}", &gift);
}
// 4. Implement the Gift trait to the gift structs
impl fmt::Display for KidsGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Gift for KidsGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}
impl fmt::Display for ElvesGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Gift for ElvesGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}
impl fmt::Display for ReindeerGift {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}
impl Gift for ReindeerGift {
    fn wrap(&mut self) {
        self.is_wrapped = true;
    }
}
pub fn main() {
    let mut kids_gift = KidsGift {
        name: "toy car".to_string(),
        is_wrapped: false,
    };
    let mut elves_gift = ElvesGift {
        name: "vertical monitor".to_string(),
        is_wrapped: false,
    };
    let mut reindeer_gift = ReindeerGift {
        name: "carrot".to_string(),
        is_wrapped: false,
    };
    prepare_gift(&mut kids_gift);
    prepare_gift(&mut elves_gift);
    prepare_gift(&mut reindeer_gift);
}
