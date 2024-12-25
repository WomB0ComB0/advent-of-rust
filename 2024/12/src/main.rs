use std::{cmp::Ordering, error::Error, ops::Deref};
// 1. Update the function signature to accept and return references to
Locations
pub fn find_most_dense_location(locations: Vec<Location>) ->
Result<Location, Box<dyn Error>> {
locations
.into_iter()
.max_by(|a, b| {
a.density()
.partial_cmp(&b.density())
.unwrap_or(Ordering::Equal)
})
.ok_or("No locations found".into())
}
pub fn find_nearest_location(locations: &[Location]) -> Result<&Location,
Box<dyn Error>> {
// 2. Find the nearest location
// Only consider locations with a density of 1000 or more
}
const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;
#[derive(Debug)]
pub struct SnowKg(pub f64);
impl SnowKg {
pub fn new(kg: f64) -> Self {
SnowKg(kg)
}
}
impl Deref for SnowKg {
type Target = f64;
fn deref(&self) -> &Self::Target {
&self.0
}
}
#[derive(Debug)]
pub struct SnowLb(pub f64);