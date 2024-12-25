# Advent of Rust 2024 - Day 16

Santa stomped into Bernard’s workshop, snatching a mug of cocoa off the table without asking. He squinted at the code from yesterday glowing on the monitor. “Ah, there it is! The Wrapped status. Beautiful work, Bernard. Gifts wrapped and ready to go.”

Bernard gave a tired nod, his tools still in hand. “Yeah, it’s done. So what now?”

Santa’s eyes gleamed as he set the mug down. “Now… we distribute those gifts. And I know exactly how. Bernard, I need a struct. A unit struct. Call it Santa.”

Bernard sighed. “Go on…”

“One method,” Santa said, his hands animated like he was pitching a startup. “Two generics: one for the recipients—kids, elves, reindeer. The other for the gifts—KidsGift, ElvesGift, ReindeerGift.”

“Two generics?” Bernard raised an eyebrow. “Why not just one?”

Santa grinned, his beard twitching with excitement. “Trait bounds, Bernard. Nothing gets my sleigh flying like a beautifully constrained generic. where clauses are Christmas magic.”

Bernard groaned. “Fine. Two generics. Single method. Anything else?”

Santa waved dismissively. “You know the drill—keep it under wraps. No one can know until it’s perfect.” He leaned in closer, eyes twinkling. “And by perfect, I mean it better compile the first time.”

As Santa vanished back into the snowy chaos, Bernard muttered, “He loves trait bounds more than Christmas itself.”

Your Mission

It's clear that Santa is now obsessed with trait bounds, so we're gonna see a lot of it here.

Here's what you gotta do

- Since we use trait bounds, we need to define a new trait named Giftable for entities that can receive gifts. Kid, Elf, and Reindeer should implement this trait.
- Giftable should have a method named receive_gift it should set the gifted field to true.
- Update the Gift trait implementation for KidsGift, ElvesGift, and ReindeerGift to include a method is_wrapped() which returns a bool to check if the gift is wrapped.
- Update the Santa struct to have a method named give_gift that accepts two arguments: a recipient which could be any of Kid, Elf, or Reindeer, and a gift which could be any of KidsGift, ElvesGift, or ReindeerGift.
- Make sure you return a Result<(), Box<dyn Error>>, if the gift is not wrapped, return an error message.

You can also have a look at the main function at the end of the code to see how it works in action.

Hints

If you're stuck or need a starting point, here are some hints to help you along the way!

- The receive_gift method should take a mutable reference to the self object since it mutates a value.
pub trait Giftable {
    fn receive_gift(&mut self);
}
- To define a function with more than one trait bound you can use the following syntax:
impl Santa {
    pub fn give_gift<R, G>(&self, recipient: &mut R, gift: &G) -> Result<(), Box<dyn Error>>
    where
        R: Giftable,
        G: Gift,
    {
        // your code here
    }
}

Santa stomped into Bernard’s workshop, snatching a mug of cocoa off the table without asking. He squinted at the code from yesterday glowing on the monitor. “Ah, there it is! The Wrapped status. Beautiful work, Bernard. Gifts wrapped and ready to go.”

Bernard gave a tired nod, his tools still in hand. “Yeah, it’s done. So what now?”

Santa’s eyes gleamed as he set the mug down. “Now… we distribute those gifts. And I know exactly how. Bernard, I need a struct. A unit struct. Call it Santa.”

Bernard sighed. “Go on…”

“One method,” Santa said, his hands animated like he was pitching a startup. “Two generics: one for the recipients—kids, elves, reindeer. The other for the gifts—KidsGift, ElvesGift, ReindeerGift.”

“Two generics?” Bernard raised an eyebrow. “Why not just one?”

Santa grinned, his beard twitching with excitement. “Trait bounds, Bernard. Nothing gets my sleigh flying like a beautifully constrained generic. where clauses are Christmas magic.”

Bernard groaned. “Fine. Two generics. Single method. Anything else?”

Santa waved dismissively. “You know the drill—keep it under wraps. No one can know until it’s perfect.” He leaned in closer, eyes twinkling. “And by perfect, I mean it better compile the first time.”

As Santa vanished back into the snowy chaos, Bernard muttered, “He loves trait bounds more than Christmas itself.”

Your Mission

It's clear that Santa is now obsessed with trait bounds, so we're gonna see a lot of it here.

Here's what you gotta do

- Since we use trait bounds, we need to define a new trait named Giftable for entities that can receive gifts. Kid, Elf, and Reindeer should implement this trait.
- Giftable should have a method named receive_gift it should set the gifted field to true.
- Update the Gift trait implementation for KidsGift, ElvesGift, and ReindeerGift to include a method is_wrapped() which returns a bool to check if the gift is wrapped.
- Update the Santa struct to have a method named give_gift that accepts two arguments: a recipient which could be any of Kid, Elf, or Reindeer, and a gift which could be any of KidsGift, ElvesGift, or ReindeerGift.
- Make sure you return a Result<(), Box<dyn Error>>, if the gift is not wrapped, return an error message.

You can also have a look at the main function at the end of the code to see how it works in action.

Hints

If you're stuck or need a starting point, here are some hints to help you along the way!

- The receive_gift method should take a mutable reference to the self object since it mutates a value.
pub trait Giftable {
    fn receive_gift(&mut self);
}
- To define a function with more than one trait bound you can use the following syntax:
impl Santa {
    pub fn give_gift<R, G>(&self, recipient: &mut R, gift: &G) -> Result<(), Box<dyn Error>>
    where
        R: Giftable,
        G: Gift,
    {
        // your code here
    }
}

## Initial Code
```rust
use std::fmt;
// For a better understanding of the problem, have a look at the end of the
file and see the `main`
// function to see how the structs are being used.
pub struct Kid {
pub name: String,
pub gifted: bool,
}
pub struct Reindeer {
pub name: String,
pub gifted: bool,
}
pub struct Elf {
pub name: String,
pub gifted: bool,
}
pub trait Giftable {
// 1. Define the trait definition
// Add a function named `receive_gift`
}
// 2. Implement `Giftable` for `Kid`, `Reindeer`, and `Elf`
pub trait Gift {
fn wrap(&mut self);
// 3. Define a function named `is_wrapped` that returns a boolean
}
// 4. Update the `Gift` trait implementation for `KidsGift`, `ElvesGift`,
and `ReindeerGift` to
//    include the `is_wrapped` function
impl Gift for KidsGift {
fn wrap(&mut self) {
self.is_wrapped = true;
}
// Update implementation
}
```
