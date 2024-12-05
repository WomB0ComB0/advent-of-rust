# Advent of Rust 2024 - Day 4

Day 4: Structifying the Naughty List

Santa burst into the dev lounge, chugging his third espresso. "Great job yesterday, nerds! The is_nice function? Chef's kiss. But now, I want structure. STRUCTURE! We’re going full-on Rustacean. I need a Kid struct—immediately!"
The elves nodded enthusiastically, their tiny laptops open, running Arch Linux with bspwm because, obviously, they were that kind of devs. One elf, started yapping, "But Santa, why a struct? Isn’t this just overengineered?"
Santa slammed the table, shaking an untouched tray of gluten-free cookies. "No! A struct means no more random strings floating around. We need to encapsulate a kid's data—name, and niceness score. Plus, we’ll need some methods to make sense of it all."
The dev elves scrambled to work. In no time, they sketched out the basic blueprint. Santa glanced at the screen. "Not bad. But I will need this extended later. Keep it modular, bros!"
The room fell silent as the elves realized the implications. This was just the beginning of Santa’s unhinged data modeling spree.
Your Task
The elves need your help to finish the Kid struct.
Here is what you need to do:

Add two variants to the Niceness enum: Nice and Naughty. Nice takes the number of good deeds.
Add two fields to the Kid struct: name of type String and niceness of type Niceness.
Move the is_nice function we created on Day 3 to an associated function of the Kid struct.
Finally, implement the new() associated function for the Kid struct.

Hints
If you're stuck, here are some hints to help you get back on track:
Open the Hints!
Use Nice(u32) to represent the number of good deeds.
To define a field in a struct, use the syntax field_name: field_type, e.g., name: String.
An associated function is defined in the impl block without the self parameter.
Associated functions are called with :: instead of ., e.g. Kid::is_nice(10, 1);
Use Self::is_nice or Kid::is_nice to call the associated function from within the impl block.

Santa burst into the dev lounge, chugging his third espresso. "Great job yesterday, nerds! The is_nice function? Chef's kiss. But now, I want structure. STRUCTURE! We’re going full-on Rustacean. I need a Kid struct—immediately!"

The elves nodded enthusiastically, their tiny laptops open, running Arch Linux with bspwm because, obviously, they were that kind of devs. One elf, started yapping, "But Santa, why a struct? Isn’t this just overengineered?"

Santa slammed the table, shaking an untouched tray of gluten-free cookies. "No! A struct means no more random strings floating around. We need to encapsulate a kid's data—name, and niceness score. Plus, we’ll need some methods to make sense of it all."

The dev elves scrambled to work. In no time, they sketched out the basic blueprint. Santa glanced at the screen. "Not bad. But I will need this extended later. Keep it modular, bros!"

The room fell silent as the elves realized the implications. This was just the beginning of Santa’s unhinged data modeling spree.

The elves need your help to finish the Kid struct.

Here is what you need to do:

If you're stuck, here are some hints to help you get back on track:

Santa burst into the dev lounge, chugging his third espresso. "Great job yesterday, nerds! The is_nice function? Chef's kiss. But now, I want structure. STRUCTURE! We’re going full-on Rustacean. I need a Kid struct—immediately!"
The elves nodded enthusiastically, their tiny laptops open, running Arch Linux with bspwm because, obviously, they were that kind of devs. One elf, started yapping, "But Santa, why a struct? Isn’t this just overengineered?"
Santa slammed the table, shaking an untouched tray of gluten-free cookies. "No! A struct means no more random strings floating around. We need to encapsulate a kid's data—name, and niceness score. Plus, we’ll need some methods to make sense of it all."
The dev elves scrambled to work. In no time, they sketched out the basic blueprint. Santa glanced at the screen. "Not bad. But I will need this extended later. Keep it modular, bros!"
The room fell silent as the elves realized the implications. This was just the beginning of Santa’s unhinged data modeling spree.
Your Task
The elves need your help to finish the Kid struct.
Here is what you need to do:

Add two variants to the Niceness enum: Nice and Naughty. Nice takes the number of good deeds.
Add two fields to the Kid struct: name of type String and niceness of type Niceness.
Move the is_nice function we created on Day 3 to an associated function of the Kid struct.
Finally, implement the new() associated function for the Kid struct.



#carbon-responsive a,#carbon-responsive a:hover{color:var(--carbon-text-color)}#carbon-responsive *{margin:unset;padding:unset;line-height:unset}#carbon-responsive{--carbon-padding:1em;--carbon-max-char:20ch;--carbon-bg-primary:hsl(0, 0%, 98%);--carbon-bg-secondary:hsl(0, 0%, 92%);--carbon-text-color:hsl(0, 0%, 20%);font-size:14px;font-family:system-ui,sans-serif;display:-webkit-box;display:-ms-flexbox;display:flex;-webkit-box-orient:vertical;-webkit-box-direction:normal;-ms-flex-direction:column;flex-direction:column;-ms-flex-wrap:wrap;flex-wrap:wrap;min-inline-size:130px;max-inline-size:400px;gap:1ex;line-height:1.5}#carbon-responsive .carbon-responsive-wrap{display:-webkit-box;display:-ms-flexbox;display:flex;-ms-flex-wrap:wrap;flex-wrap:wrap;padding:var(--carbon-padding);gap:var(--carbon-padding);border:solid 1px var(--carbon-bg-secondary);background-color:var(--carbon-bg-primary)}#carbon-responsive a{text-decoration:none}#carbon-responsive .carbon-img{-webkit-box-flex:0;-ms-flex:0 0 130px;flex:0 0 130px}#carbon-responsive .carbon-img img{display:block}#carbon-responsive .carbon-text{-webkit-box-flex:1;-ms-flex-positive:1;flex-grow:1;-ms-flex-preferred-size:var(--carbon-max-char);flex-basis:var(--carbon-max-char);line-height:1.4;text-align:left}#carbon-responsive .carbon-poweredby{font-size:.85em;text-align:right;opacity:.5}



	
			
		
		
			Get 20% off your first 3 months of BugHerd. Start Shipping Faster Today.
		

ads via Carbon

Hints
If you're stuck, here are some hints to help you get back on track:
Open the Hints!
Use Nice(u32) to represent the number of good deeds.
To define a field in a struct, use the syntax field_name: field_type, e.g., name: String.
An associated function is defined in the impl block without the self parameter.
Associated functions are called with :: instead of ., e.g. Kid::is_nice(10, 1);
Use Self::is_nice or Kid::is_nice to call the associated function from within the impl block.

Santa burst into the dev lounge, chugging his third espresso. "Great job yesterday, nerds! The is_nice function? Chef's kiss. But now, I want structure. STRUCTURE! We’re going full-on Rustacean. I need a Kid struct—immediately!"

The elves nodded enthusiastically, their tiny laptops open, running Arch Linux with bspwm because, obviously, they were that kind of devs. One elf, started yapping, "But Santa, why a struct? Isn’t this just overengineered?"

Santa slammed the table, shaking an untouched tray of gluten-free cookies. "No! A struct means no more random strings floating around. We need to encapsulate a kid's data—name, and niceness score. Plus, we’ll need some methods to make sense of it all."

The dev elves scrambled to work. In no time, they sketched out the basic blueprint. Santa glanced at the screen. "Not bad. But I will need this extended later. Keep it modular, bros!"

The room fell silent as the elves realized the implications. This was just the beginning of Santa’s unhinged data modeling spree.

The elves need your help to finish the Kid struct.

Here is what you need to do:

If you're stuck, here are some hints to help you get back on track:

## Initial Code
```rust
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;
#[derive(Debug, PartialEq)] // needed for tests
pub enum Niceness {
// Create the enum variants `Nice` and `Naughty`
// Variant `Nice` is a tuple struct that holds the number of good deeds
}
pub struct Kid {
// Add a field `name` of type `String`
// and `niceness` field of type `Niceness`
// Make all fields public
}
// Move yesterday's function to an associated function in the struct
pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
if good_deeds == 0 && bad_deeds == 0 {
return false;
}
let good_deeds = good_deeds as f32 * GOOD_WEIGHT;
let bad_deeds = bad_deeds as f32 * BAD_WEIGHT;
let ratio = good_deeds / (good_deeds + bad_deeds);
ratio >= 0.75
}
impl Kid {
pub fn new(name: String, good_deeds: u32, bad_deeds: u32) -> Kid {
// Return a Kid instance
}
}
```
