# Advent of Rust 2024 - Day 10

The elves huddled in the workshop, their chatter buzzing with excitement.

"Have you seen the new ChatGPT model that just launched?" one elf asked, adjusting their tiny glasses.

"I did! But did you hear? It costs $200 a month!" another replied, shaking their head in disbelief.

“Two hundred dollars? That’s a fortune!” chimed in a third, tossing a tinsel garland into a storage box.

The discussion heated up as they debated whether the advanced features were worth the hefty price tag, but their conversation was abruptly interrupted by a booming voice.

“I can’t deref my snowballs! What’s going on?” Santa’s frustrated shout echoed through the workshop.

The elves froze mid-conversation, their heads snapping toward the jolly old man standing by his snowball workstation. He wore an expression of pure consternation as he jabbed a finger at the terminal screen displaying a stream of cryptic error messages.

One of the more tech-savvy elves hesitantly stepped forward. “Santa, you can just use snowball.0 to access the first value in the tuple.”

Santa’s frown deepened, and he waved a hand dismissively. “Nonsense! I need to deref all the structs. Every single one!”

The elves weren’t sure what Santa was coding, but they knew better than to argue with him when he was in debugging mode. Scrambling into action, they began reviewing his codebase, determined to find a solution before the big man lost his holiday cheer.

Your Mission

The code you wrote yesterday is a good start, but Santa needs easy access to the struct fields, instead of accessing them like this snowball.0, he wants to access them like this *snowball.

Here is what you need to do:

- Implement Deref for SnowKg.
- Implement Deref for SnowLb.
- Implement Deref for Snowball.
- Return the inner value of the tuple struct when dereferencing.

Hints

If you’re stuck or need a starting point, here are some hints to help you along the way!

- Import the Deref trait from the std::ops module: use std::ops::Deref;.
- Implement the Deref trait for SnowKg, SnowLb, and Snowball. e.g., impl Deref for SnowKg { ... }.
- Define the associated type Target as the type of the inner value of the tuple struct. e.g.
type Target = f64;
- Implement the deref method for each struct. e.g.
fn deref(&self) -> &Self::Target {
    &self.0
}

Import the Deref trait from the std::ops module: use std::ops::Deref;.

Implement the Deref trait for SnowKg, SnowLb, and Snowball. e.g., impl Deref for SnowKg { ... }.

Define the associated type Target as the type of the inner value of the tuple struct. e.g.

Implement the deref method for each struct. e.g.

The elves huddled in the workshop, their chatter buzzing with excitement.

"Have you seen the new ChatGPT model that just launched?" one elf asked, adjusting their tiny glasses.

"I did! But did you hear? It costs $200 a month!" another replied, shaking their head in disbelief.

“Two hundred dollars? That’s a fortune!” chimed in a third, tossing a tinsel garland into a storage box.

The discussion heated up as they debated whether the advanced features were worth the hefty price tag, but their conversation was abruptly interrupted by a booming voice.

“I can’t deref my snowballs! What’s going on?” Santa’s frustrated shout echoed through the workshop.

The elves froze mid-conversation, their heads snapping toward the jolly old man standing by his snowball workstation. He wore an expression of pure consternation as he jabbed a finger at the terminal screen displaying a stream of cryptic error messages.

One of the more tech-savvy elves hesitantly stepped forward. “Santa, you can just use snowball.0 to access the first value in the tuple.”

Santa’s frown deepened, and he waved a hand dismissively. “Nonsense! I need to deref all the structs. Every single one!”

The elves weren’t sure what Santa was coding, but they knew better than to argue with him when he was in debugging mode. Scrambling into action, they began reviewing his codebase, determined to find a solution before the big man lost his holiday cheer.

Your Mission

The code you wrote yesterday is a good start, but Santa needs easy access to the struct fields, instead of accessing them like this snowball.0, he wants to access them like this *snowball.

Here is what you need to do:

- Implement Deref for SnowKg.
- Implement Deref for SnowLb.
- Implement Deref for Snowball.
- Return the inner value of the tuple struct when dereferencing.

Hints

If you’re stuck or need a starting point, here are some hints to help you along the way!

- Import the Deref trait from the std::ops module: use std::ops::Deref;.
- Implement the Deref trait for SnowKg, SnowLb, and Snowball. e.g., impl Deref for SnowKg { ... }.
- Define the associated type Target as the type of the inner value of the tuple struct. e.g.
type Target = f64;
- Implement the deref method for each struct. e.g.
fn deref(&self) -> &Self::Target {
    &self.0
}

Import the Deref trait from the std::ops module: use std::ops::Deref;.

Implement the Deref trait for SnowKg, SnowLb, and Snowball. e.g., impl Deref for SnowKg { ... }.

Define the associated type Target as the type of the inner value of the tuple struct. e.g.

Implement the deref method for each struct. e.g.

## Initial Code
```rust
const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;
pub struct SnowKg(pub f64);
impl SnowKg {
pub fn new(kg: f64) -> Self {
SnowKg(kg)
}
}
// 1. Implement the `Deref` trait for `SnowKg`
// 2. Implement the `Deref` trait for `SnowLb`
// 3. Implement the `Deref` trait for `Snowball`
pub struct SnowLb(pub f64);
impl SnowLb {
pub fn new(lb: f64) -> Self {
SnowLb(lb)
}
}
pub struct Snowball(pub i64);
impl Snowball {
pub fn new(snowballs: i64) -> Self {
Snowball(snowballs)
}
}
impl From<SnowKg> for Snowball {
fn from(kg: SnowKg) -> Self {
let snowballs = (*kg / SNOWBALL_WEIGHT_KG).round() as i64;
Snowball(snowballs)
}
}
impl From<SnowLb> for Snowball {
fn from(lb: SnowLb) -> Self {
let snowballs = (*lb / SNOWBALL_WEIGHT_LB).round() as i64;
Snowball(snowballs)
}
}
```
