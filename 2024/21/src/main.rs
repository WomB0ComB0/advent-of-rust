pub struct TempFile {}
impl TempFile {
pub fn new() -> Result<Self, std::io::Error> {
// Your code here...
}
pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
// Your code here...
}
pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
// Your code here...
}
pub fn path(&self) -> &PathBuf {
&self.file_path
}
pub fn file(&self) -> &File {
&self.file
}
}