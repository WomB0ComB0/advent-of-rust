pub fn send_message_to_santa() -> String {
    String::from("I've been nice")
}

pub fn main() {
    let message = send_message_to_santa();
    println!("{}", message);
}