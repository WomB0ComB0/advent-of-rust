# Advent of Rust 2024 - Day 23

"Santa, we need to talk!" Snowball stammered, holding a stack of printouts that smelled vaguely of desperation.

Santa looked up, his patience thinner than Rudolph’s battery life. "What now?"

Bernard, the senior elf, didn’t mince words. "It’s the Naughty-Nice List system. Legacy code. Written in JavaScript."

Santa froze. "Not even TypeScript?"

Bernard shook his head. "Plain JavaScript. var everywhere. No types. No safety. It’s a dumpster fire. Someone even polyfilled Promise with copy-paste."

Santa slammed his mug down. "So Naughty kids get PS5s, and Nice kids get socks because JavaScript?"

Snowball nodded, sweating. "It’s worse, Santa. There’s no validation. The Nice List has a SELECT * injection, and someone added console.log("Merry Christmas, LOL"); in production!"

Santa pinched the bridge of his nose. "Rewriting it in Rust is our only hope. No globals, no runtime panics—safety guaranteed. Bernard, make it happen."

"But Santa," Bernard hesitated, "starting from scratch this late… it’s risky."

Santa leaned forward, eyes blazing. "Riskier than trusting JavaScript on Christmas Eve? I don’t care how late it is. Write it in Rust."

Your Mission

Implement a SantaList struct that uses a HashMap to store children’s names as keys and their behaviors (true for nice, false for naughty) as values.

The SantaList struct should have a single field:

- records - a HashMap<String, bool> to store children’s names and behaviors.

The struct should have the following methods and associated functions:

- new - Create a new SantaList instance.
- add - Add a child's name and behavior to the list.
- remove - Remove a child from the list.
- get - Retrieve a child's behavior.
- count - Count the number of nice and naughty children as a tuple (nice, naughty)
- list_by_behavior - Retrieve a list of children based on their behavior as Vec<String>

Hints

If you’re unsure where to start, take a look at these tips:

- Using HashMap: Import it with use std::collections::HashMap;. Use HashMap::new() to create an empty map.
- Adding/Updating: Use insert(key, value) to add or update an entry.
- Querying: Use get(key) to retrieve values. It returns an Option<bool>.
- Removing: Use remove(key) to delete an entry and get its value if it exists.
- Counting: Use values() with .filter() to count nice/naughty children:

Using HashMap: Import it with use std::collections::HashMap;. Use HashMap::new() to create an empty map.

Adding/Updating: Use insert(key, value) to add or update an entry.

Querying: Use get(key) to retrieve values. It returns an Option<bool>.

Removing: Use remove(key) to delete an entry and get its value if it exists.

Counting: Use values() with .filter() to count nice/naughty children:

"Santa, we need to talk!" Snowball stammered, holding a stack of printouts that smelled vaguely of desperation.

Santa looked up, his patience thinner than Rudolph’s battery life. "What now?"

Bernard, the senior elf, didn’t mince words. "It’s the Naughty-Nice List system. Legacy code. Written in JavaScript."

Santa froze. "Not even TypeScript?"

Bernard shook his head. "Plain JavaScript. var everywhere. No types. No safety. It’s a dumpster fire. Someone even polyfilled Promise with copy-paste."

Santa slammed his mug down. "So Naughty kids get PS5s, and Nice kids get socks because JavaScript?"

Snowball nodded, sweating. "It’s worse, Santa. There’s no validation. The Nice List has a SELECT * injection, and someone added console.log("Merry Christmas, LOL"); in production!"

Santa pinched the bridge of his nose. "Rewriting it in Rust is our only hope. No globals, no runtime panics—safety guaranteed. Bernard, make it happen."

"But Santa," Bernard hesitated, "starting from scratch this late… it’s risky."

Santa leaned forward, eyes blazing. "Riskier than trusting JavaScript on Christmas Eve? I don’t care how late it is. Write it in Rust."

Your Mission

Implement a SantaList struct that uses a HashMap to store children’s names as keys and their behaviors (true for nice, false for naughty) as values.

The SantaList struct should have a single field:

- records - a HashMap<String, bool> to store children’s names and behaviors.

The struct should have the following methods and associated functions:

- new - Create a new SantaList instance.
- add - Add a child's name and behavior to the list.
- remove - Remove a child from the list.
- get - Retrieve a child's behavior.
- count - Count the number of nice and naughty children as a tuple (nice, naughty)
- list_by_behavior - Retrieve a list of children based on their behavior as Vec<String>

Hints

If you’re unsure where to start, take a look at these tips:

- Using HashMap: Import it with use std::collections::HashMap;. Use HashMap::new() to create an empty map.
- Adding/Updating: Use insert(key, value) to add or update an entry.
- Querying: Use get(key) to retrieve values. It returns an Option<bool>.
- Removing: Use remove(key) to delete an entry and get its value if it exists.
- Counting: Use values() with .filter() to count nice/naughty children:

Using HashMap: Import it with use std::collections::HashMap;. Use HashMap::new() to create an empty map.

Adding/Updating: Use insert(key, value) to add or update an entry.

Querying: Use get(key) to retrieve values. It returns an Option<bool>.

Removing: Use remove(key) to delete an entry and get its value if it exists.

Counting: Use values() with .filter() to count nice/naughty children:

## Initial Code
```rust
pub struct SantaList {
// 1. Define the records field
}
impl SantaList {
// 2. Implement the new method
// 3. Implement the add method
// 4. Implement the remove method
// 5. Implement the get method
// 6. Implement the count method
// 7. Implement the list_by_behavior method
}
pub fn main() {
let mut santa_list = SantaList::new();
santa_list.add("Alice", true);
santa_list.add("Bob", false);
santa_list.add("Charlie", true);
if let Some(behavior) = santa_list.get("Alice") {
println!(
"Alice is on the {} list.",
if behavior { "Nice" } else { "Naughty" }
);
}
let (nice, naughty) = santa_list.count();
println!(
"Santa's list contains {} nice and {} naughty children.",
nice, naughty
);
let nice_list = santa_list.list_by_behavior(true);
println!("Nice children: {:?}", nice_list);
let naughty_list = santa_list.list_by_behavior(false);
println!("Naughty children: {:?}", naughty_list);
santa_list.remove("Bob");
```
