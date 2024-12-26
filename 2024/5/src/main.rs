pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;
#[derive(Debug, PartialEq)]
pub enum Niceness {
    Nice(u32),
    Naughty,
}
pub struct Kid {
    pub name: String,
    pub niceness: Niceness,
}
impl Kid {
    pub fn parse_row(csv_row: &str) -> Result<Kid, &'static str> {
        // 🎁 Transform the CSV row into a Kid struct for Santa's list!
        // 🎅 Expected CSV: "Name,GoodDeeds,BadDeeds"
        //    Example: "Alice,3,1" -> name: "Alice", good_deeds: 3, bad_deeds: 1
        // 🎁 Your code here! 🎁
        let mut csv_row = csv_row.split(",");
        let name = csv_row.next().ok_or("missing name")?;
        let good_deeds = csv_row
            .next()
            .ok_or("missing good deeds")?
            .parse()
            .map_err(|_| "can't parse good deeds")?;
        let bad_deeds = csv_row
            .next()
            .ok_or("missing bad deeds")?
            .parse()
            .map_err(|_| "can't parse bad deeds")?;
        Ok(Self::new(name.to_string(), good_deeds, bad_deeds))
    }
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Self {
        let niceness = if Self::is_nice(good_deeds, bad_deeds) {
            Niceness::Nice(good_deeds)
        } else {
            Niceness::Naughty
        };
        Self { name, niceness }
    }
    pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
        100 * good_deeds >= 75 * (good_deeds + 2 * bad_deeds) && good_deeds > 0
    }
}

pub fn main() {
    println!("{}", Kid::is_nice(10, 2));
}
