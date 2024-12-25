pub struct LogQuery<'a> {
logs: &'a Vec<String>,
}
// 1. Finish the implementation of LogQuery
impl<'a>
// 2. Create a public associated function named `new()` that will take a
reference to a vector of strings
//
// 3. Create a public method named `search` that accepts a string slice and
finds it from the logs and
//    returns a vector of references to those logs.