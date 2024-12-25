# Advent of Rust 2024 - Day 20

“IS THE SLEIGH PARKED, READY, OR FLYING? WHY DOESN’T ANYONE KNOW?”

“Uh… shouldn’t you know? Didn't we use the type state pattern?” Bernard spoke up.

Santa's face twitched. "Of course, we used the type state pattern! But does anyone have a quick way to check the state right now? There are some sleighs in the air, it's hard to keep track on all of them."

Blitzen, lounging in a corner, snorted. “Sounds like y'all need to add a status() method.”

Prancer’s ears perked up. “Yeah! A single method we can call to figure out the state, no matter which one it’s in.”

Santa stroked his beard, muttering. “Fine. Add it. And make sure it’s compile-time safe. If I get a wrong answer, someone’s spending Christmas hand-delivering packets in binary.”

Your Mission

It's time to give Santa what he wants: a single status() method that works in all states of the sleigh (Empty, Ready, and Flying), returning the current state as a string.

Here's the plan:

- "Empty" when the sleigh is empty.
- "Ready" when the sleigh is loaded and ready to fly.
- "Flying" when the sleigh is in the air.

Here's how Santa wants to the the API:

Hints

If you’re unsure where to start, take a look at these tips:

- Use a trait to define shared behavior across all states. For example:
pub trait State {
    fn status() -> &'static str;
}
- Implement the trait for each state. e.g.
impl State for Empty {
    fn status() -> &'static str {
        "Empty"
    }
}
- Implement a method for all types that implement the State trait. e.g.
impl<T: State> Sleigh<T> {
  pub fn status(&self) -> &'static str {
      T::status()
  }
}

Use a trait to define shared behavior across all states. For example:

Implement the trait for each state. e.g.

Implement a method for all types that implement the State trait. e.g.

“IS THE SLEIGH PARKED, READY, OR FLYING? WHY DOESN’T ANYONE KNOW?”

“Uh… shouldn’t you know? Didn't we use the type state pattern?” Bernard spoke up.

Santa's face twitched. "Of course, we used the type state pattern! But does anyone have a quick way to check the state right now? There are some sleighs in the air, it's hard to keep track on all of them."

Blitzen, lounging in a corner, snorted. “Sounds like y'all need to add a status() method.”

Prancer’s ears perked up. “Yeah! A single method we can call to figure out the state, no matter which one it’s in.”

Santa stroked his beard, muttering. “Fine. Add it. And make sure it’s compile-time safe. If I get a wrong answer, someone’s spending Christmas hand-delivering packets in binary.”

Your Mission

It's time to give Santa what he wants: a single status() method that works in all states of the sleigh (Empty, Ready, and Flying), returning the current state as a string.

Here's the plan:

- "Empty" when the sleigh is empty.
- "Ready" when the sleigh is loaded and ready to fly.
- "Flying" when the sleigh is in the air.

Here's how Santa wants to the the API:

Hints

If you’re unsure where to start, take a look at these tips:

- Use a trait to define shared behavior across all states. For example:
pub trait State {
    fn status() -> &'static str;
}
- Implement the trait for each state. e.g.
impl State for Empty {
    fn status() -> &'static str {
        "Empty"
    }
}
- Implement a method for all types that implement the State trait. e.g.
impl<T: State> Sleigh<T> {
  pub fn status(&self) -> &'static str {
      T::status()
  }
}

Use a trait to define shared behavior across all states. For example:

Implement the trait for each state. e.g.

Implement a method for all types that implement the State trait. e.g.

## Initial Code
```rust
use std::marker::PhantomData;
pub struct Empty;
pub struct Ready;
pub struct Flying;
// TODO: Define the `status` method for all states
pub struct Sleigh<T> {
// This is only public for testing purposes
// In real-world scenarios, this should be private
pub state: PhantomData<T>,
}
impl Sleigh<Empty> {
pub fn new() -> Self {
Self { state: PhantomData }
}
pub fn load(self) -> Sleigh<Ready> {
Sleigh { state: PhantomData }
}
}
impl Sleigh<Ready> {
pub fn take_off(self) -> Sleigh<Flying> {
Sleigh { state: PhantomData }
}
pub fn unload(self) -> Sleigh<Empty> {
Sleigh { state: PhantomData }
}
}
impl Sleigh<Flying> {
pub fn land(self) -> Sleigh<Ready> {
Sleigh { state: PhantomData }
}
}
```
