# Advent of Rust 2024 - Day 1

Day 1

⚠️ Important Note
We use clone() in this example, but we are not claiming it is the optimal approach. The goal here is to present a solution that helps illustrate Rust's ownership system in a simple way. As the story unfolds in the next challenge, we'll dive into more efficient patterns. This example is intentionally simplified for the sake of education and storytelling.
Enjoy!

The Story
It's 1st December and the countdown has just begun. The elves are busy preparing for the Christmas and Santa is busy checking the list of children who have been good this year. It was supposed to be a smooth day until all of a sudden two of santa's elves burst into Santa's office with a problem.
“Santa!” one of the elves shouted. “The code won’t compile! We’ve hit a wall, and it’s all Rust’s fault!”
Santa, sipping his triple-shot peppermint latte, raised an eyebrow. “Rust’s fault? Or your fault?”
“It’s the ownership rules!” the other elve blurted. “I think we violated them, we’re used to Python, where variables just... work. Look at this!”
The elves tried their best, here is what they've written so far:
fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message);
 
    println!("{}", gift_message);
}
 
fn attach_message_to_present(message: String) {
    println!("The present now has this message: {}", message);
}
However, the code won't compile. Can you help the elves attach the message to the present and print the message too?
Maybe... if there was only a way to get a clone of the message and pass it to the function. 🤔
Hints

The elves want you to only use clone() to fix the code.

We use clone() in this example, but we are not claiming it is the optimal approach. The goal here is to present a solution that helps illustrate Rust's ownership system in a simple way. As the story unfolds in the next challenge, we'll dive into more efficient patterns. This example is intentionally simplified for the sake of education and storytelling.

Enjoy!

It's 1st December and the countdown has just begun. The elves are busy preparing for the Christmas and Santa is busy checking the list of children who have been good this year. It was supposed to be a smooth day until all of a sudden two of santa's elves burst into Santa's office with a problem.

“Santa!” one of the elves shouted. “The code won’t compile! We’ve hit a wall, and it’s all Rust’s fault!”

Santa, sipping his triple-shot peppermint latte, raised an eyebrow. “Rust’s fault? Or your fault?”

“It’s the ownership rules!” the other elve blurted. “I think we violated them, we’re used to Python, where variables just... work. Look at this!”

The elves tried their best, here is what they've written so far:

However, the code won't compile. Can you help the elves attach the message to the present and print the message too?

Maybe... if there was only a way to get a clone of the message and pass it to the function. 🤔

⚠️ Important Note
We use clone() in this example, but we are not claiming it is the optimal approach. The goal here is to present a solution that helps illustrate Rust's ownership system in a simple way. As the story unfolds in the next challenge, we'll dive into more efficient patterns. This example is intentionally simplified for the sake of education and storytelling.
Enjoy!



#carbon-responsive a,#carbon-responsive a:hover{color:var(--carbon-text-color)}#carbon-responsive *{margin:unset;padding:unset;line-height:unset}#carbon-responsive{--carbon-padding:1em;--carbon-max-char:20ch;--carbon-bg-primary:hsl(0, 0%, 98%);--carbon-bg-secondary:hsl(0, 0%, 92%);--carbon-text-color:hsl(0, 0%, 20%);font-size:14px;font-family:system-ui,sans-serif;display:-webkit-box;display:-ms-flexbox;display:flex;-webkit-box-orient:vertical;-webkit-box-direction:normal;-ms-flex-direction:column;flex-direction:column;-ms-flex-wrap:wrap;flex-wrap:wrap;min-inline-size:130px;max-inline-size:400px;gap:1ex;line-height:1.5}#carbon-responsive .carbon-responsive-wrap{display:-webkit-box;display:-ms-flexbox;display:flex;-ms-flex-wrap:wrap;flex-wrap:wrap;padding:var(--carbon-padding);gap:var(--carbon-padding);border:solid 1px var(--carbon-bg-secondary);background-color:var(--carbon-bg-primary)}#carbon-responsive a{text-decoration:none}#carbon-responsive .carbon-img{-webkit-box-flex:0;-ms-flex:0 0 130px;flex:0 0 130px}#carbon-responsive .carbon-img img{display:block}#carbon-responsive .carbon-text{-webkit-box-flex:1;-ms-flex-positive:1;flex-grow:1;-ms-flex-preferred-size:var(--carbon-max-char);flex-basis:var(--carbon-max-char);line-height:1.4;text-align:left}#carbon-responsive .carbon-poweredby{font-size:.85em;text-align:right;opacity:.5}



	
			
		
		
			Listen to chill LoFi Beats now.
		

ads via Carbon

The Story
It's 1st December and the countdown has just begun. The elves are busy preparing for the Christmas and Santa is busy checking the list of children who have been good this year. It was supposed to be a smooth day until all of a sudden two of santa's elves burst into Santa's office with a problem.
“Santa!” one of the elves shouted. “The code won’t compile! We’ve hit a wall, and it’s all Rust’s fault!”
Santa, sipping his triple-shot peppermint latte, raised an eyebrow. “Rust’s fault? Or your fault?”
“It’s the ownership rules!” the other elve blurted. “I think we violated them, we’re used to Python, where variables just... work. Look at this!”
The elves tried their best, here is what they've written so far:
fn main() {
    let gift_message = String::from("Merry Christmas! Enjoy your gift!");
    attach_message_to_present(gift_message);
 
    println!("{}", gift_message);
}
 
fn attach_message_to_present(message: String) {
    println!("The present now has this message: {}", message);
}
However, the code won't compile. Can you help the elves attach the message to the present and print the message too?
Maybe... if there was only a way to get a clone of the message and pass it to the function. 🤔
Hints

The elves want you to only use clone() to fix the code.

We use clone() in this example, but we are not claiming it is the optimal approach. The goal here is to present a solution that helps illustrate Rust's ownership system in a simple way. As the story unfolds in the next challenge, we'll dive into more efficient patterns. This example is intentionally simplified for the sake of education and storytelling.

Enjoy!

It's 1st December and the countdown has just begun. The elves are busy preparing for the Christmas and Santa is busy checking the list of children who have been good this year. It was supposed to be a smooth day until all of a sudden two of santa's elves burst into Santa's office with a problem.

“Santa!” one of the elves shouted. “The code won’t compile! We’ve hit a wall, and it’s all Rust’s fault!”

Santa, sipping his triple-shot peppermint latte, raised an eyebrow. “Rust’s fault? Or your fault?”

“It’s the ownership rules!” the other elve blurted. “I think we violated them, we’re used to Python, where variables just... work. Look at this!”

The elves tried their best, here is what they've written so far:

However, the code won't compile. Can you help the elves attach the message to the present and print the message too?

Maybe... if there was only a way to get a clone of the message and pass it to the function. 🤔

## Initial Code
```rust
pub fn main() {
let gift_message = String::from("Merry Christmas! Enjoy your gift!");
attach_message_to_present(gift_message); // <-- Add a `.clone()` here!
println!("{}", gift_message);
}
pub fn attach_message_to_present(message: String) {
println!("The present now has this message: {}", message);
}
```
