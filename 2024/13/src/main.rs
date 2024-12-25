// Import the necessary modules
pub enum ParseError {
// 1. Add variants here (read description)
}
// 2. Implement the Error trait for ParseError
pub struct Kid {
pub name: String,
pub niceness: Niceness,
}
impl Kid {
pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
let niceness = if Self::is_nice(good_deeds, bad_deeds) {
Niceness::Nice(good_deeds)
} else {
Niceness::Naughty
};
Kid { name, niceness }
}
pub fn parse_row(csv_row: &str) -> Result<Kid, ParseError> {
// 3. Update the code to return meaningful errors
let mut fields = csv_row.split(',');
let name = fields.next().ok_or("No name")?.to_string();
let good_deeds = fields
.next()
.ok_or("No good deeds")?
.parse::<u32>()
.map_err(|_| "Invalid good deeds")?;
let bad_deeds = fields
.next()
.ok_or("No bad deeds")?
.parse::<u32>()
.map_err(|_| "Invalid bad deeds")?;
Ok(Kid::new(name, good_deeds, bad_deeds))
}
pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
if good_deeds == 0 && bad_deeds == 0 {
return false;