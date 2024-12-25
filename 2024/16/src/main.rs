use std::fmt;
// For a better understanding of the problem, have a look at the end of the
file and see the `main`
// function to see how the structs are being used.
pub struct Kid {
pub name: String,
pub gifted: bool,
}
pub struct Reindeer {
pub name: String,
pub gifted: bool,
}
pub struct Elf {
pub name: String,
pub gifted: bool,
}
pub trait Giftable {
// 1. Define the trait definition
// Add a function named `receive_gift`
}
// 2. Implement `Giftable` for `Kid`, `Reindeer`, and `Elf`
pub trait Gift {
fn wrap(&mut self);
// 3. Define a function named `is_wrapped` that returns a boolean
}
// 4. Update the `Gift` trait implementation for `KidsGift`, `ElvesGift`,
and `ReindeerGift` to
//    include the `is_wrapped` function
impl Gift for KidsGift {
fn wrap(&mut self) {
self.is_wrapped = true;
}
// Update implementation
}