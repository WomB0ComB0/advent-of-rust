pub struct SantaList {
// 1. Define the records field
}
impl SantaList {
// 2. Implement the new method
// 3. Implement the add method
// 4. Implement the remove method
// 5. Implement the get method
// 6. Implement the count method
// 7. Implement the list_by_behavior method
}
pub fn main() {
let mut santa_list = SantaList::new();
santa_list.add("Alice", true);
santa_list.add("Bob", false);
santa_list.add("Charlie", true);
if let Some(behavior) = santa_list.get("Alice") {
println!(
"Alice is on the {} list.",
if behavior { "Nice" } else { "Naughty" }
);
}
let (nice, naughty) = santa_list.count();
println!(
"Santa's list contains {} nice and {} naughty children.",
nice, naughty
);
let nice_list = santa_list.list_by_behavior(true);
println!("Nice children: {:?}", nice_list);
let naughty_list = santa_list.list_by_behavior(false);
println!("Naughty children: {:?}", naughty_list);
santa_list.remove("Bob");