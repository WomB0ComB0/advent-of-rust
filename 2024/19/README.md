# Advent of Rust 2024 - Day 19

“Last year, someone called take_off mid-flight, and we ended up performing air donuts over Siberia. Never. Again.”

“It was kind of cool, though.” Prancer muttered.

Santa whirled. “Cool? COOL? I had to explain to a reindeer rescue hotline why Blitzen was tangled in GPS satellites! No more cowboy coding. We’re doing this with the type state pattern.”

Bernard hesitated. “Uh, what’s wrong with just adding a check for is_flying?”

Santa's glare could have melted the North Pole. “Oh, brilliant idea, Bernard. Let’s just trust runtime checks! No. The sleigh will know its state: parked, flying, or landed. You call take_off while flying? The compiler won’t even let you. No crashes, no ISS collisions, no lawsuits.”

Blitzen snorted. “Sounds over-engineered.”

"BLITZEN, do you want to be the first reindeer to test the emergency eject system? Get to work. I want this sleigh state-safe by tonight!"

Your Mission

We don't want to end up with a sleigh in the wrong state again and we certainly don't want the take_off method to be available when the sleigh is already flying.

And Santa doesn't want any runtime checks, so it must all be checked at compile time.

Here's what you need to do:

- Finish the Sleigh struct to represent the state of the sleigh.
- The Sleigh struct must have 3 states Empty, Ready and Flying.

There are a few methods we want to define for each state:

- new() an associated function that creates a new Sleigh in the Empty state.
- load() a method that transitions the Sleigh from Empty to Ready.

- take_off() a method that transitions the Sleigh from Ready to Flying.
- unload() a method that transitions the Sleigh from Ready to Empty.

- land() a method that transitions the Sleigh from Flying to Ready.

Hints

If you're stuck or need a starting point, here are some hints to help you along the way!

- Create three structs for each state Empty, Ready and Flying.
pub struct Empty;
pub struct Ready;
pub struct Flying;
- You can create a generic struct that represents the state of the sleigh. e.g.
struct Sleigh<State> {
  // State must be used
}
- Use PhantomData to ensure that the state is used in the struct without consuming any memory.
use std::marker::PhantomData;
 
struct Sleigh<State> {
    _state: PhantomData<State>,
}
- Implement the methods for each state. e.g.
impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }
 
    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

Create three structs for each state Empty, Ready and Flying.

You can create a generic struct that represents the state of the sleigh. e.g.

Use PhantomData to ensure that the state is used in the struct without consuming any memory.

Implement the methods for each state. e.g.

“Last year, someone called take_off mid-flight, and we ended up performing air donuts over Siberia. Never. Again.”

“It was kind of cool, though.” Prancer muttered.

Santa whirled. “Cool? COOL? I had to explain to a reindeer rescue hotline why Blitzen was tangled in GPS satellites! No more cowboy coding. We’re doing this with the type state pattern.”

Bernard hesitated. “Uh, what’s wrong with just adding a check for is_flying?”

Santa's glare could have melted the North Pole. “Oh, brilliant idea, Bernard. Let’s just trust runtime checks! No. The sleigh will know its state: parked, flying, or landed. You call take_off while flying? The compiler won’t even let you. No crashes, no ISS collisions, no lawsuits.”

Blitzen snorted. “Sounds over-engineered.”

"BLITZEN, do you want to be the first reindeer to test the emergency eject system? Get to work. I want this sleigh state-safe by tonight!"

Your Mission

We don't want to end up with a sleigh in the wrong state again and we certainly don't want the take_off method to be available when the sleigh is already flying.

And Santa doesn't want any runtime checks, so it must all be checked at compile time.

Here's what you need to do:

- Finish the Sleigh struct to represent the state of the sleigh.
- The Sleigh struct must have 3 states Empty, Ready and Flying.

There are a few methods we want to define for each state:

- new() an associated function that creates a new Sleigh in the Empty state.
- load() a method that transitions the Sleigh from Empty to Ready.

- take_off() a method that transitions the Sleigh from Ready to Flying.
- unload() a method that transitions the Sleigh from Ready to Empty.

- land() a method that transitions the Sleigh from Flying to Ready.

Hints

If you're stuck or need a starting point, here are some hints to help you along the way!

- Create three structs for each state Empty, Ready and Flying.
pub struct Empty;
pub struct Ready;
pub struct Flying;
- You can create a generic struct that represents the state of the sleigh. e.g.
struct Sleigh<State> {
  // State must be used
}
- Use PhantomData to ensure that the state is used in the struct without consuming any memory.
use std::marker::PhantomData;
 
struct Sleigh<State> {
    _state: PhantomData<State>,
}
- Implement the methods for each state. e.g.
impl Sleigh<Empty> {
    pub fn new() -> Self {
        Self { state: PhantomData }
    }
 
    pub fn load(self) -> Sleigh<Ready> {
        Sleigh { state: PhantomData }
    }
}

Create three structs for each state Empty, Ready and Flying.

You can create a generic struct that represents the state of the sleigh. e.g.

Use PhantomData to ensure that the state is used in the struct without consuming any memory.

Implement the methods for each state. e.g.

## Initial Code
```rust
// 1. We have 3 states:
// - Empty
// - Ready
// - Flying
// 2. Finish the Seligh struct definition
pub struct Sleigh
// 3. Write the Sleigh Implementations for all states
```
