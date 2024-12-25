# Advent of Rust 2024 - Day 12

Santa was livid. His face was redder than Rudolph’s nose after a Red Bull binge. The elves had tried the sleigh's navigation system again, running the find_best_location function. Instead of landing them in a cozy, gift-ready location, the sleigh had whisked them right back to the North Pole.

“I told you to land in the best spot, not bring me back to this frosty hellhole!” Santa bellowed. His voice echoed across the frozen tundra, sending a nearby snowman into early retirement.

Bernard cautiously stepped forward. “Erm… Sir, the function seems to be, uh, calculating the densest snow coverage but doesn’t account for it's distance from us." Santa’s eyes narrowed. “I said best place to land, not snowiest! Who do I look like, a yeti?”

“Show me the code,” Santa growled, extending a gloved hand that looked ready to smash a keyboard.

Bernard shakily handed him the tablet. “Here it is, boss.”

Santa squinted at the screen, his rage escalating as he scanned the lines. “FOR HOLLY JOLLY’S SAKE—are you not using references again?! A consuming iterator? Do you want to crash the sleigh into a snowbank?!”

The elves collectively winced, knowing that Santa's rants about proper memory management could last for hours.

Santa shoved the tablet back at Bernard. “Fix it, or I’m replacing the sleigh team with drones. And, use references for the love of all that’s holly and jolly, Bernard! I won’t tolerate another memory leak on Christmas Eve!”

Your Mission

The function find_best_location is not behaving as expected. But the code might still be useful, so the elves don’t want to scrap it entirely, instead they renamed it to find_most_dense_location.

Here is what you need to do for today:

- Update the functions signature to accept a &[Location] instead of a Vec<Location> and return a reference to a Location instead of an owned value.
- Write the code for the other function find_nearest_location that will return the nearest location to the current location which is (x = 0, y = 0). Ignore the z coordinate.
- There should be a minimum of density of 1000.0 to be considered a good location. If there weren't any, return an Error.

Hints

If you’re stuck or need a starting point, here are some hints to help you along the way!

- To make the function accept a reference to the Vec<Location> you can use, &[Location] as the argument type.
- For the return type, you should return a reference to a Location instead of an owned value. Result<&Location, Box<dyn Error>>
- In order to return the type as a reference, the iterator should not be consumed anymore, so instead of into_iter use iter().
- Filter the locations that have a density greater than 1000.0 using the filter function from the Iterator trait. e.g. locations.iter().filter(|a| a.density() >= 1000.0).
- To find the nearest location to the current location, you can use the min_by function from the Iterator trait.
- The min_by function takes a closure that compares two elements and returns the one that should be considered the minimum.
- You can get the distance between the origin (0, 0) and (x, y) by using the hypot method from the f64 type. e.g. a.x.hypot(a.y). This will return the distance between the origin and the location a.
- Use a partial_cmp just like the other function to compare the distances between two locations.
- Convert the Option<&Location> to a Result<&Location, Box<dyn Error>> using the ok_or method.

To make the function accept a reference to the Vec<Location> you can use, &[Location] as the argument type.

For the return type, you should return a reference to a Location instead of an owned value. Result<&Location, Box<dyn Error>>

In order to return the type as a reference, the iterator should not be consumed anymore, so instead of into_iter use iter().

Filter the locations that have a density greater than 1000.0 using the filter function from the Iterator trait. e.g. locations.iter().filter(|a| a.density() >= 1000.0).

To find the nearest location to the current location, you can use the min_by function from the Iterator trait.

The min_by function takes a closure that compares two elements and returns the one that should be considered the minimum.

You can get the distance between the origin (0, 0) and (x, y) by using the hypot method from the f64 type. e.g. a.x.hypot(a.y). This will return the distance between the origin and the location a.

Use a partial_cmp just like the other function to compare the distances between two locations.

Convert the Option<&Location> to a Result<&Location, Box<dyn Error>> using the ok_or method.

Santa was livid. His face was redder than Rudolph’s nose after a Red Bull binge. The elves had tried the sleigh's navigation system again, running the find_best_location function. Instead of landing them in a cozy, gift-ready location, the sleigh had whisked them right back to the North Pole.

“I told you to land in the best spot, not bring me back to this frosty hellhole!” Santa bellowed. His voice echoed across the frozen tundra, sending a nearby snowman into early retirement.

Bernard cautiously stepped forward. “Erm… Sir, the function seems to be, uh, calculating the densest snow coverage but doesn’t account for it's distance from us." Santa’s eyes narrowed. “I said best place to land, not snowiest! Who do I look like, a yeti?”

“Show me the code,” Santa growled, extending a gloved hand that looked ready to smash a keyboard.

Bernard shakily handed him the tablet. “Here it is, boss.”

Santa squinted at the screen, his rage escalating as he scanned the lines. “FOR HOLLY JOLLY’S SAKE—are you not using references again?! A consuming iterator? Do you want to crash the sleigh into a snowbank?!”

The elves collectively winced, knowing that Santa's rants about proper memory management could last for hours.

Santa shoved the tablet back at Bernard. “Fix it, or I’m replacing the sleigh team with drones. And, use references for the love of all that’s holly and jolly, Bernard! I won’t tolerate another memory leak on Christmas Eve!”

Your Mission

The function find_best_location is not behaving as expected. But the code might still be useful, so the elves don’t want to scrap it entirely, instead they renamed it to find_most_dense_location.

Here is what you need to do for today:

- Update the functions signature to accept a &[Location] instead of a Vec<Location> and return a reference to a Location instead of an owned value.
- Write the code for the other function find_nearest_location that will return the nearest location to the current location which is (x = 0, y = 0). Ignore the z coordinate.
- There should be a minimum of density of 1000.0 to be considered a good location. If there weren't any, return an Error.

Hints

If you’re stuck or need a starting point, here are some hints to help you along the way!

- To make the function accept a reference to the Vec<Location> you can use, &[Location] as the argument type.
- For the return type, you should return a reference to a Location instead of an owned value. Result<&Location, Box<dyn Error>>
- In order to return the type as a reference, the iterator should not be consumed anymore, so instead of into_iter use iter().
- Filter the locations that have a density greater than 1000.0 using the filter function from the Iterator trait. e.g. locations.iter().filter(|a| a.density() >= 1000.0).
- To find the nearest location to the current location, you can use the min_by function from the Iterator trait.
- The min_by function takes a closure that compares two elements and returns the one that should be considered the minimum.
- You can get the distance between the origin (0, 0) and (x, y) by using the hypot method from the f64 type. e.g. a.x.hypot(a.y). This will return the distance between the origin and the location a.
- Use a partial_cmp just like the other function to compare the distances between two locations.
- Convert the Option<&Location> to a Result<&Location, Box<dyn Error>> using the ok_or method.

To make the function accept a reference to the Vec<Location> you can use, &[Location] as the argument type.

For the return type, you should return a reference to a Location instead of an owned value. Result<&Location, Box<dyn Error>>

In order to return the type as a reference, the iterator should not be consumed anymore, so instead of into_iter use iter().

Filter the locations that have a density greater than 1000.0 using the filter function from the Iterator trait. e.g. locations.iter().filter(|a| a.density() >= 1000.0).

To find the nearest location to the current location, you can use the min_by function from the Iterator trait.

The min_by function takes a closure that compares two elements and returns the one that should be considered the minimum.

You can get the distance between the origin (0, 0) and (x, y) by using the hypot method from the f64 type. e.g. a.x.hypot(a.y). This will return the distance between the origin and the location a.

Use a partial_cmp just like the other function to compare the distances between two locations.

Convert the Option<&Location> to a Result<&Location, Box<dyn Error>> using the ok_or method.

## Initial Code
```rust
use std::{cmp::Ordering, error::Error, ops::Deref};
// 1. Update the function signature to accept and return references to
Locations
pub fn find_most_dense_location(locations: Vec<Location>) ->
Result<Location, Box<dyn Error>> {
locations
.into_iter()
.max_by(|a, b| {
a.density()
.partial_cmp(&b.density())
.unwrap_or(Ordering::Equal)
})
.ok_or("No locations found".into())
}
pub fn find_nearest_location(locations: &[Location]) -> Result<&Location,
Box<dyn Error>> {
// 2. Find the nearest location
// Only consider locations with a density of 1000 or more
}
const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;
#[derive(Debug)]
pub struct SnowKg(pub f64);
impl SnowKg {
pub fn new(kg: f64) -> Self {
SnowKg(kg)
}
}
impl Deref for SnowKg {
type Target = f64;
fn deref(&self) -> &Self::Target {
&self.0
}
}
#[derive(Debug)]
pub struct SnowLb(pub f64);
```
