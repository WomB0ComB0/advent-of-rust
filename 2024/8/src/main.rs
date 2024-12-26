use std::{fs::File, io::Write};
pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}
impl<'a> LogQuery<'a> {
    pub fn new(logs: &'a Vec<String>) -> Self {
        LogQuery { logs }
    }
    pub fn search(&self, keyword: &str) -> Vec<&'a String> {
        self.logs
            .iter()
            .filter(|log| log.contains(keyword))
            .collect()
    }
    pub fn export_to_file(&self, keyword: &str, path: &str) -> std::io::Result<()> {
        let mut file = File::create(path)?;
        for log in self.search(keyword) {
            writeln!(file, "{}", log)?;
        }
        Ok(())
    }
}

fn main() {
    let logs = vec![
        "error: something went wrong".to_string(),
        "warning: something might go wrong".to_string(),
        "info: everything is fine".to_string(),
    ];
    let query = LogQuery::new(&logs);
    let results = query.search("error");
    println!("{:?}", results);
}
