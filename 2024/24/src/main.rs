use std::sync::Arc;
use std::thread;
pub trait SleighTask {
fn describe(&self) -> String;
}
pub struct SantaSleighQueue {
records: // 1. Should store the tasks
}
impl SantaSleighQueue {
// 2. Define the `new` constructor
// 3. Define the `enqueue` method
// 4. Define the `get_task` method
}
pub struct ElfTask {
// 5. Define the fields
}
impl ElfTask {
// 6. Define the `new` constructor
}
impl SleighTask for ElfTask {
fn describe(&self) -> String {
format!("Elf task: {} (urgency {})", self.name, self.urgency)
}
}
pub struct ReindeerTask {
// 7. Define the fields
}
impl ReindeerTask {
// 8. Define the `new` constructor
}
impl SleighTask for ReindeerTask {
fn describe(&self) -> String {
format!("Reindeer task: {} ({} kg)", self.name, self.weight)
}