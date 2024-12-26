pub struct LogQuery<'a> {
    logs: &'a Vec<String>,
}
// 1. Finish the implementation of LogQuery
impl<'a> LogQuery<'a> {
    // 2. Create a public associated function named `new()` that will take a reference to a vector of strings
    pub fn new(values: &'a Vec<String>) -> LogQuery {
        LogQuery { logs: values }
    }
    // 3. Create a public method named `search` that accepts a string slice and finds it from the logs and returns a vector of references to those logs.
    pub fn search(&self, term: &str) -> Vec<&'a str> {
        let mut result: Vec<&str> = Vec::new();
        for log in self.logs.iter() {
            if log.contains(term) {
                result.push(log);
            }
        }
        result
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
