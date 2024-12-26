use std::fmt::{self, Display, Formatter};
#[derive(Debug)]
pub enum ParseError {
    NoName,
    NoGoodDeeds,
    NoBadDeeds,
    InvalidGoodDeeds,
    InvalidBadDeeds,
}
impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        use ParseError as PE;
        match self {
            PE::NoName => f.write_str("Name field is missing"),
            PE::NoGoodDeeds => f.write_str("Good deeds field is missing"),
            PE::NoBadDeeds => f.write_str("Bad deeds field is missing"),
            PE::InvalidGoodDeeds => f.write_str("Good deeds value is invalid"),
            PE::InvalidBadDeeds => f.write_str("Bad deeds value is invalid"),
        }
    }
}
impl std::error::Error for ParseError {}
// 2. Implement the Error trait for ParseError
#[derive(Debug)]
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
        use ParseError as Error;
        // 3. Update the code to return meaningful errors
        let mut fields = csv_row.split(',');
        let name = fields
            .next()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .ok_or(Error::NoName)?
            .to_string();
        let good_deeds = fields
            .next()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .ok_or(Error::NoGoodDeeds)?
            .parse::<u32>()
            .map_err(|_| Error::InvalidGoodDeeds)?;
        let bad_deeds = fields
            .next()
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .ok_or(Error::NoBadDeeds)?
            .parse::<u32>()
            .map_err(|_| Error::InvalidBadDeeds)?;
        Ok(Kid::new(name, good_deeds, bad_deeds))
    }
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        if good_deeds == 0 && bad_deeds == 0 {
            return false;
        }
        let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
        let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;
        let ratio = good_deeds / (good_deeds + bad_deeds);
        ratio >= 0.75
    }
}
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;
#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}

pub fn main() {
    let kid = Kid::new(String::from("John"), 10, 5);
    println!("{:?}", kid);
}