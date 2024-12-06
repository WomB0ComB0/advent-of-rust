pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;
#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
    // Create the enum variants `Nice` and `Naughty`
    // Variant `Nice` is a tuple struct that holds the number of good deeds
    Nice(u32),
    Naughty,
}
pub struct Kid {
    // Add a field `name` of type `String`
    // and `niceness` field of type `Niceness`
    // Make all fields public
    pub name: String,
    pub niceness: Niceness,
}
impl Kid {
    pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
        let niceness = match Kid::is_nice(good_deeds, bad_deeds) {
            true => Niceness::Nice(good_deeds),
            false => Niceness::Naughty,
        };
        Kid { name, niceness}
    }
    // Move yesterday's function to an associated function in the struct
pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
    100*good_deeds >= 75*(1*good_deeds+2*bad_deeds) && good_deeds > 0
}
}

pub fn main() {
    println!("{}", Kid::is_nice(10, 2));
}