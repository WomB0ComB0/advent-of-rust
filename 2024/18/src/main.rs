pub struct Sleigh {
color: String,
engine: String,
gift_capacity: u32,
magical_enhancements: bool,
}
#[derive(Clone)]
pub struct SleighBuilder {
// Define the fields of SleighBuilder here
}
impl SleighBuilder {
// Your code here...
}
// Don't Change this implementation
// It is used for the tests
impl Sleigh {
pub fn color(&self) -> &str {
&self.color
}
pub fn engine(&self) -> &str {
&self.engine
}
pub fn gift_capacity(&self) -> u32 {
self.gift_capacity
}
pub fn magical_enhancements(&self) -> bool {
self.magical_enhancements
}
}
pub fn main() {
let sleigh = SleighBuilder::new()
.color("gold")
.engine("magic")
.gift_capacity(350)
.magical_enhancements()
.build();
assert_eq!(sleigh.color(), "gold");