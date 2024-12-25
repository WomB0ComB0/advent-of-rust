use std::{error::Error, ops::Deref};
#[derive(Debug, Clone)]
pub struct Location {
pub x: f64,
pub y: f64,
pub z: f64,
pub area: f64,
pub snow: Snowball,
}
impl Location {
// 1. Implement the `new()` method.
// Parameters (must be in order):
// - x: f64
// - y: f64
// - z: f64
// - area: f64
// - snow: Either `SnowKg`, `SnowLb` or `Snowball`
pub fn density(&self) -> f64 {
// 2. Implement the `density()` method.
// Calculation: snow / area
// all area is in one unit, so don't worry about the unit conversion.
// Return 0.0 if the area is 0.0.
}
}
pub fn find_best_location(locations: Vec<Location>) -> Result<Location,
Box<dyn Error>> {
// 3. Find the location with the highest snow density.
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