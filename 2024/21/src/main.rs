use std::env;
use std::path::PathBuf;
use std::fs::{File, OpenOptions, remove_file, read_to_string};
use std::time::SystemTime;
use std::io::Write;

pub struct TempFile {
    file_path: PathBuf,
    file: File,
}

impl TempFile {
    pub fn new() -> Result<Self, std::io::Error> {
        let current = SystemTime::now()
                        .duration_since(SystemTime::UNIX_EPOCH)
                        .unwrap()
                        .subsec_nanos()
                        .to_string();
        let file_path = env::temp_dir().join(format!("{:?}", current));
        let file = OpenOptions::new()
                        .read(true)
                        .write(true)
                        .create(true)
                        .open(&file_path)?;
        Ok(TempFile {
            file_path,
            file
        })
    }
    pub fn write(&mut self, data: &[u8]) -> Result<(), std::io::Error> {
        let _ = &self.file.write_all(data)?;
        Ok(())
    }
    pub fn read_to_string(&mut self) -> Result<String, std::io::Error> {
        read_to_string(&self.file_path)
    }
    pub fn path(&self) -> &PathBuf {
        &self.file_path
    }
    pub fn file(&self) -> &File {
        &self.file
    }
}
impl Drop for TempFile {
    fn drop(&mut self) {
        let _ = remove_file(&self.file_path);
    }
}

pub fn main() {
    let mut temp_file = TempFile::new().unwrap();
    temp_file.write(b"Hello, world!").unwrap();
    let content = temp_file.read_to_string().unwrap();
    println!("{}", content);
}
