# Advent of Rust 2024 - Day 22

“Blitzen, do you realize every single read_to_string() call triggers a fresh heap allocation? Do you think we’ve got infinite memory lying around?”

Here, I called it a few times and the process is taking up 1.5GB of memory, you think this is acceptable?

Blitzen glanced up, nonchalant. “Heap allocations happen, Santa. That’s life.”

Santa’s eyes narrowed. “Do you want the nightmare of Day 1 all over again? That disaster was just a simple string clone, and it nearly broke us. This? This is worse. Way worse. I don’t want to end up manually running drop() just to keep things from spiraling into chaos!”

Blitzen stiffened, finally taking the hint. “Okay, okay. What do you want me to do?”

Prancer cut in cautiously. “Why not cache the file content on every write? And return a reference to the cached content on every read?”

Blitzen nodded slowly. “Yeah, that’ll work. No more redundant allocations, no stale reads, and we don’t touch drop() at all.”

Santa’s glare softened—marginally. “Good. Make it lean, make it fast, and make sure it’s ready by sunrise. We don't have much time, Christmas is just around the corner.”

Your Mission

Santa's files are huge, and they use a lot of memory, To prevent that we need to create a new method named read_to_str that will return a string slice instead of an owned value.

Here is what you need to do:

- Create a new field called content
- Update the write method to update the content on every write
- Complete the read_from_cache method to return a string slice instead of an owned value

If you’re unsure where to start, take a look at these tips:

- Create a new field called content to store the file content
pub struct TempFile {
    file: File,
    file_path: PathBuf,
    content: Vec<u8>,
}
- Update the field in file writes, use to_vec() to turn a &[u8] to a Vec<u8>
self.content = data.to_vec();
- Use the std::str::from_utf8 function to convert a &[u8] to a &str
std::str::from_utf8(&self.content).unwrap_or("")

Create a new field called content to store the file content

Update the field in file writes, use to_vec() to turn a &[u8] to a Vec<u8>

Use the std::str::from_utf8 function to convert a &[u8] to a &str

“Blitzen, do you realize every single read_to_string() call triggers a fresh heap allocation? Do you think we’ve got infinite memory lying around?”

Here, I called it a few times and the process is taking up 1.5GB of memory, you think this is acceptable?

Blitzen glanced up, nonchalant. “Heap allocations happen, Santa. That’s life.”

Santa’s eyes narrowed. “Do you want the nightmare of Day 1 all over again? That disaster was just a simple string clone, and it nearly broke us. This? This is worse. Way worse. I don’t want to end up manually running drop() just to keep things from spiraling into chaos!”

Blitzen stiffened, finally taking the hint. “Okay, okay. What do you want me to do?”

Prancer cut in cautiously. “Why not cache the file content on every write? And return a reference to the cached content on every read?”

Blitzen nodded slowly. “Yeah, that’ll work. No more redundant allocations, no stale reads, and we don’t touch drop() at all.”

Santa’s glare softened—marginally. “Good. Make it lean, make it fast, and make sure it’s ready by sunrise. We don't have much time, Christmas is just around the corner.”

Your Mission

Santa's files are huge, and they use a lot of memory, To prevent that we need to create a new method named read_to_str that will return a string slice instead of an owned value.

Here is what you need to do:

- Create a new field called content
- Update the write method to update the content on every write
- Complete the read_from_cache method to return a string slice instead of an owned value

If you’re unsure where to start, take a look at these tips:

- Create a new field called content to store the file content
pub struct TempFile {
    file: File,
    file_path: PathBuf,
    content: Vec<u8>,
}
- Update the field in file writes, use to_vec() to turn a &[u8] to a Vec<u8>
self.content = data.to_vec();
- Use the std::str::from_utf8 function to convert a &[u8] to a &str
std::str::from_utf8(&self.content).unwrap_or("")

Create a new field called content to store the file content

Update the field in file writes, use to_vec() to turn a &[u8] to a Vec<u8>

Use the std::str::from_utf8 function to convert a &[u8] to a &str

## Initial Code
```rust
use std::fs::{remove_file, File, OpenOptions};
use std::io::{Read, Seek, Write};
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};
pub struct TempFile {
file_path: PathBuf,
file: File,
// 1. Add a new field named `content` with a valid type
}
impl TempFile {
pub fn new() -> Result<Self, std::io::Error> {
let temp_dir = std::env::temp_dir();
let random_number: u64 = {
let start = SystemTime::now();
let since_epoch = start.duration_since(UNIX_EPOCH).unwrap();
since_epoch.as_nanos() as u64
};
let file_name = format!("tempfile-{}.tmp", random_number);
let file_path = temp_dir.join(file_name);
let file = OpenOptions::new()
.read(true)
.write(true)
.create(true)
.open(&file_path)?;
Ok(Self { file_path, file })
}
// 2. Change this method to update the `content` field on every write
pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
self.file.write_all(data)?;
Ok(())
}
pub fn read_from_cache(&self) -> &str {
// 3. Update this method to return the content as a string slice
}
pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
let mut buf = String::new();
self.file.seek(std::io::SeekFrom::Start(0))?;
```
