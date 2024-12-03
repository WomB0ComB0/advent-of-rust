# Advent of Rust 2024 - Day 3

Day 3: Restoring the Nice List

The elves were in high spirits. For the first time in centuries, yesterday’s code review had eradicated every unnecessary heap allocation in Santa’s list-checking algorithm. “Finally,” yapped an elf sipping a Red Bull mocktail, “no more unnecessary allocations, no more clones”
The workshop buzzed with excitement as the DevOps elves live-streamed the successful merge on ElfHub. Even Blitzen was chill for once, reclining by the server rack, listening to lofi beats.
But the joy didn’t last.
Santa stormed in, his energy somewhere between a VC pitch gone wrong and a meme that didn’t land on X. His face was redder than Rudolph’s nose.
“WHY,” he roared, “IS THE NICE LIST COMPLETELY EMPTY?”
The elves froze.
“What do you mean, empty?” stammered an elf. “It compiled perfectly last night—”
Santa cut them off. “LOOK! Not a single kid on the Nice list. Did you break the weights? Are we back to random clones and allocations?!” He slammed his candy-cane laptop onto the nearest desk, the screen glaring with the issue.

The Root Cause
The elves scrambled to debug. An intrepid junior elf opened the codebase. “Wait,” they muttered. “The weights are fine, but... the ratio logic’s busted.”
They pointed at the critical function, it was written in Python and didn't look so good.
"Anyone want to debug?" Santa added, his voice a mix of hope and despair. Surprisingly, nobody answered, it seemed like the code was written so badly that nobody wanted to touch it.
"Forget it," Santa said, "We'll re-write everything in Rust. I heard it's the new trend."

Your Mission
Help the elves re-write the is_nice function in Rust. Santa needs the Nice list back before Christmas Eve.
The is_nice function accepts two arguments:

good_deeds: u32: The number of good deeds a kid has done.
bad_deeds: u32: The number of bad deeds a kid has done.

Calculating the ratio
To calculate the ratio, follow this logic:
ratio = good_deeds / (good_deeds + bad_deeds)
But there's a catch!
Bad deeds are weighted more heavily than good deeds (twice as much). So, the final ratio is calculated as:
ratio = good_deeds / (good_deeds + (2 * bad_deeds))
After you find the ratio, you'll need to check if the kid is nice. A kid is considered nice if the ratio is greater than or equal to 0.75, if nice return true, otherwise return false.
Santa’s counting on you. Save Christmas and keep the Nice list free of data breaches — and, hopefully, Santa himself.
Requirements

If both good_deeds and bad_deeds are 0, the kid is naughty by default.
The function should return a bool value.


Hints
If you're stuck, here are some hints to help you get back on track:
Open the Hints!

Use the as keywords to convert the u32 (unsigned 32-bit integer) to a f64 (floating-point number) before doing any numerical operations. e.g. good_deeds as f64


Remember to use parentheses to ensure the correct order of operations.


Remember to return a bool value from the function.

The elves were in high spirits. For the first time in centuries, yesterday’s code review had eradicated every unnecessary heap allocation in Santa’s list-checking algorithm. “Finally,” yapped an elf sipping a Red Bull mocktail, “no more unnecessary allocations, no more clones”

The workshop buzzed with excitement as the DevOps elves live-streamed the successful merge on ElfHub. Even Blitzen was chill for once, reclining by the server rack, listening to lofi beats.

But the joy didn’t last.

Santa stormed in, his energy somewhere between a VC pitch gone wrong and a meme that didn’t land on X. His face was redder than Rudolph’s nose.

“WHY,” he roared, “IS THE NICE LIST COMPLETELY EMPTY?”

The elves froze.

“What do you mean, empty?” stammered an elf. “It compiled perfectly last night—”

Santa cut them off. “LOOK! Not a single kid on the Nice list. Did you break the weights? Are we back to random clones and allocations?!” He slammed his candy-cane laptop onto the nearest desk, the screen glaring with the issue.

The elves scrambled to debug. An intrepid junior elf opened the codebase. “Wait,” they muttered. “The weights are fine, but... the ratio logic’s busted.”

They pointed at the critical function, it was written in Python and didn't look so good.

"Anyone want to debug?" Santa added, his voice a mix of hope and despair. Surprisingly, nobody answered, it seemed like the code was written so badly that nobody wanted to touch it.

"Forget it," Santa said, "We'll re-write everything in Rust. I heard it's the new trend."

Help the elves re-write the is_nice function in Rust. Santa needs the Nice list back before Christmas Eve.

The is_nice function accepts two arguments:

To calculate the ratio, follow this logic:

Bad deeds are weighted more heavily than good deeds (twice as much). So, the final ratio is calculated as:

After you find the ratio, you'll need to check if the kid is nice. A kid is considered nice if the ratio is greater than or equal to 0.75, if nice return true, otherwise return false.

Santa’s counting on you. Save Christmas and keep the Nice list free of data breaches — and, hopefully, Santa himself.

If you're stuck, here are some hints to help you get back on track:

Use the as keywords to convert the u32 (unsigned 32-bit integer) to a f64 (floating-point number) before doing any numerical operations. e.g. good_deeds as f64

Remember to use parentheses to ensure the correct order of operations.

Remember to return a bool value from the function.

The elves were in high spirits. For the first time in centuries, yesterday’s code review had eradicated every unnecessary heap allocation in Santa’s list-checking algorithm. “Finally,” yapped an elf sipping a Red Bull mocktail, “no more unnecessary allocations, no more clones”
The workshop buzzed with excitement as the DevOps elves live-streamed the successful merge on ElfHub. Even Blitzen was chill for once, reclining by the server rack, listening to lofi beats.
But the joy didn’t last.
Santa stormed in, his energy somewhere between a VC pitch gone wrong and a meme that didn’t land on X. His face was redder than Rudolph’s nose.
“WHY,” he roared, “IS THE NICE LIST COMPLETELY EMPTY?”
The elves froze.
“What do you mean, empty?” stammered an elf. “It compiled perfectly last night—”
Santa cut them off. “LOOK! Not a single kid on the Nice list. Did you break the weights? Are we back to random clones and allocations?!” He slammed his candy-cane laptop onto the nearest desk, the screen glaring with the issue.

The Root Cause
The elves scrambled to debug. An intrepid junior elf opened the codebase. “Wait,” they muttered. “The weights are fine, but... the ratio logic’s busted.”
They pointed at the critical function, it was written in Python and didn't look so good.
"Anyone want to debug?" Santa added, his voice a mix of hope and despair. Surprisingly, nobody answered, it seemed like the code was written so badly that nobody wanted to touch it.
"Forget it," Santa said, "We'll re-write everything in Rust. I heard it's the new trend."



#carbon-responsive a,#carbon-responsive a:hover{color:var(--carbon-text-color)}#carbon-responsive *{margin:unset;padding:unset;line-height:unset}#carbon-responsive{--carbon-padding:1em;--carbon-max-char:20ch;--carbon-bg-primary:hsl(0, 0%, 98%);--carbon-bg-secondary:hsl(0, 0%, 92%);--carbon-text-color:hsl(0, 0%, 20%);font-size:14px;font-family:system-ui,sans-serif;display:-webkit-box;display:-ms-flexbox;display:flex;-webkit-box-orient:vertical;-webkit-box-direction:normal;-ms-flex-direction:column;flex-direction:column;-ms-flex-wrap:wrap;flex-wrap:wrap;min-inline-size:130px;max-inline-size:400px;gap:1ex;line-height:1.5}#carbon-responsive .carbon-responsive-wrap{display:-webkit-box;display:-ms-flexbox;display:flex;-ms-flex-wrap:wrap;flex-wrap:wrap;padding:var(--carbon-padding);gap:var(--carbon-padding);border:solid 1px var(--carbon-bg-secondary);background-color:var(--carbon-bg-primary)}#carbon-responsive a{text-decoration:none}#carbon-responsive .carbon-img{-webkit-box-flex:0;-ms-flex:0 0 130px;flex:0 0 130px}#carbon-responsive .carbon-img img{display:block}#carbon-responsive .carbon-text{-webkit-box-flex:1;-ms-flex-positive:1;flex-grow:1;-ms-flex-preferred-size:var(--carbon-max-char);flex-basis:var(--carbon-max-char);line-height:1.4;text-align:left}#carbon-responsive .carbon-poweredby{font-size:.85em;text-align:right;opacity:.5}



	
			
		
		
			Build the tech skills you need to take control of your career. (The salary boost’s an added bonus.)
		

ads via Carbon

Your Mission
Help the elves re-write the is_nice function in Rust. Santa needs the Nice list back before Christmas Eve.
The is_nice function accepts two arguments:

good_deeds: u32: The number of good deeds a kid has done.
bad_deeds: u32: The number of bad deeds a kid has done.

Calculating the ratio
To calculate the ratio, follow this logic:
ratio = good_deeds / (good_deeds + bad_deeds)
But there's a catch!
Bad deeds are weighted more heavily than good deeds (twice as much). So, the final ratio is calculated as:
ratio = good_deeds / (good_deeds + (2 * bad_deeds))
After you find the ratio, you'll need to check if the kid is nice. A kid is considered nice if the ratio is greater than or equal to 0.75, if nice return true, otherwise return false.
Santa’s counting on you. Save Christmas and keep the Nice list free of data breaches — and, hopefully, Santa himself.
Requirements

If both good_deeds and bad_deeds are 0, the kid is naughty by default.
The function should return a bool value.


Hints
If you're stuck, here are some hints to help you get back on track:
Open the Hints!

Use the as keywords to convert the u32 (unsigned 32-bit integer) to a f64 (floating-point number) before doing any numerical operations. e.g. good_deeds as f64


Remember to use parentheses to ensure the correct order of operations.


Remember to return a bool value from the function.

The elves were in high spirits. For the first time in centuries, yesterday’s code review had eradicated every unnecessary heap allocation in Santa’s list-checking algorithm. “Finally,” yapped an elf sipping a Red Bull mocktail, “no more unnecessary allocations, no more clones”

The workshop buzzed with excitement as the DevOps elves live-streamed the successful merge on ElfHub. Even Blitzen was chill for once, reclining by the server rack, listening to lofi beats.

But the joy didn’t last.

Santa stormed in, his energy somewhere between a VC pitch gone wrong and a meme that didn’t land on X. His face was redder than Rudolph’s nose.

“WHY,” he roared, “IS THE NICE LIST COMPLETELY EMPTY?”

The elves froze.

“What do you mean, empty?” stammered an elf. “It compiled perfectly last night—”

Santa cut them off. “LOOK! Not a single kid on the Nice list. Did you break the weights? Are we back to random clones and allocations?!” He slammed his candy-cane laptop onto the nearest desk, the screen glaring with the issue.

The elves scrambled to debug. An intrepid junior elf opened the codebase. “Wait,” they muttered. “The weights are fine, but... the ratio logic’s busted.”

They pointed at the critical function, it was written in Python and didn't look so good.

"Anyone want to debug?" Santa added, his voice a mix of hope and despair. Surprisingly, nobody answered, it seemed like the code was written so badly that nobody wanted to touch it.

"Forget it," Santa said, "We'll re-write everything in Rust. I heard it's the new trend."

Help the elves re-write the is_nice function in Rust. Santa needs the Nice list back before Christmas Eve.

The is_nice function accepts two arguments:

To calculate the ratio, follow this logic:

Bad deeds are weighted more heavily than good deeds (twice as much). So, the final ratio is calculated as:

After you find the ratio, you'll need to check if the kid is nice. A kid is considered nice if the ratio is greater than or equal to 0.75, if nice return true, otherwise return false.

Santa’s counting on you. Save Christmas and keep the Nice list free of data breaches — and, hopefully, Santa himself.

If you're stuck, here are some hints to help you get back on track:

Use the as keywords to convert the u32 (unsigned 32-bit integer) to a f64 (floating-point number) before doing any numerical operations. e.g. good_deeds as f64

Remember to use parentheses to ensure the correct order of operations.

Remember to return a bool value from the function.

## Initial Code
```rust
// We need to find the nice and naughty kids for santa
// Each good deed is worth 1 point and each bad deed is worth 2 points
pub const GOOD_WEIGHT: f32 = 1.0;
pub const BAD_WEIGHT: f32 = 2.0;
pub fn is_nice(good_deeds: u32, bad_deeds: u32) -> bool {
// Calculate the ratio of good deeds to total deeds
// Any ratio greater than 0.75 is considered nice
// e.g. 10 good deeds and 2 bad deeds =
// (10 * 1) / (10 * 1) + (2 * 2) = 10 / 14 = 0.714... (not nice)
// If both good and bad deeds are 0, the child is naughty
}
```
