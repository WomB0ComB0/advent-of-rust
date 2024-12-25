# Advent of Rust 2024 - Day 14

Santa glared at the terminal, baffled. “A simple println!! That’s all I needed! Why won’t it work?!” He tugged his beard, fuming. The elves had been cranking out code under his “motivational” leadership, but now even Santa’s Rust “expertise” was falling short.

Bernard, the Lead Elf, leaned over Santa’s shoulder, squinting at the code. “Wait… are those gifts for elves and reindeer too? Are you planning to give everyone presents this year?”

Santa slammed the laptop shut, his cheeks reddening. “It’s supposed to be a surprise! If Prancer finds out, she’ll livestream the announcement before the code even works. Now fix this, Bernard—discreetly.”

Bernard crossed his arms, smirking. “Santa, that’s ambitious. But this code is spaghetti. I’ll fix it—but you owe the elves an extra cocoa break.”

Santa growled. “Fine! Just fix it! And not a word to anyone until it’s done.”

Bernard chuckled, cracking his knuckles. “You got it, boss!”

Your Mission

If Bernard solves this problem, it will be likely that the elves and reindeer get gifts from Santa too, it'll be their lucky year, you just need to write a function so that Santa easily can log the gifts, the function should be able to take in either KidsGift, ElvesGift or ReindeerGift and print them out.

Make sure you update the signature of the display_gift function to accept any of the types.

Hints

If you're stuck or need a starting point, here are some hints to help you along the way!

- Use trait bounds to accept only those types that implement a specific trait.
- Import the Display trait using use std::fmt::Display.
- Implement the trait for each gift type.
- In order for a type to be displayable, it needs to implement the Display trait.
- Here is an example of a trait bound:
fn my_fn<T: fmt::Display>(arg: T) {
    println!("{}", arg);
}

Santa glared at the terminal, baffled. “A simple println!! That’s all I needed! Why won’t it work?!” He tugged his beard, fuming. The elves had been cranking out code under his “motivational” leadership, but now even Santa’s Rust “expertise” was falling short.

Bernard, the Lead Elf, leaned over Santa’s shoulder, squinting at the code. “Wait… are those gifts for elves and reindeer too? Are you planning to give everyone presents this year?”

Santa slammed the laptop shut, his cheeks reddening. “It’s supposed to be a surprise! If Prancer finds out, she’ll livestream the announcement before the code even works. Now fix this, Bernard—discreetly.”

Bernard crossed his arms, smirking. “Santa, that’s ambitious. But this code is spaghetti. I’ll fix it—but you owe the elves an extra cocoa break.”

Santa growled. “Fine! Just fix it! And not a word to anyone until it’s done.”

Bernard chuckled, cracking his knuckles. “You got it, boss!”

Your Mission

If Bernard solves this problem, it will be likely that the elves and reindeer get gifts from Santa too, it'll be their lucky year, you just need to write a function so that Santa easily can log the gifts, the function should be able to take in either KidsGift, ElvesGift or ReindeerGift and print them out.

Make sure you update the signature of the display_gift function to accept any of the types.

Hints

If you're stuck or need a starting point, here are some hints to help you along the way!

- Use trait bounds to accept only those types that implement a specific trait.
- Import the Display trait using use std::fmt::Display.
- Implement the trait for each gift type.
- In order for a type to be displayable, it needs to implement the Display trait.
- Here is an example of a trait bound:
fn my_fn<T: fmt::Display>(arg: T) {
    println!("{}", arg);
}

## Initial Code
```rust
pub struct KidsGift {
pub name: String,
}
pub struct ElvesGift {
pub name: String,
}
pub struct ReindeerGift {
pub name: String,
}
pub fn display_gift(gift: Unknown) {
println!("{}", gift);
}
pub fn main() {
let kids_gift = KidsGift {
name: "toy car".to_string(),
};
let elves_gift = ElvesGift {
name: "vertical monitor".to_string(),
};
let reindeer_gift = ReindeerGift {
name: "carrot".to_string(),
};
display_gift(&kids_gift);
display_gift(&elves_gift);
display_gift(&reindeer_gift);
}
```
