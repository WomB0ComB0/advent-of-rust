// Ensure all relevant items are marked with `pub` keyword
const CHRISTMAS_EMOJIS: [char; 4] = ['🎅', '🤶', '🎄', '🎁'];
// Your Solution here ...
pub fn main() {
let emails = vec![
"rudolph.therapysessions@northpole.com".to_string(),
"elfhr.complaint@northpole.urgent".to_string(),
"santas.rage.management@christmaschaos.noel".to_string(),
"overtimepay.never@elfexploitation.workshop".to_string(),
"mrs.claus.divorce.lawyer@northpole.legal".to_string(),
"reindeer.workers.comp@antler.insurance".to_string(),
"naughty.list.revenge@santasecrets.com".to_string(),
"workshop.ptsd.support@elves.anonymous".to_string(),
"performance.anxiety@santa.breakdown".to_string(),
"existential.crisis@northpole.void".to_string(),
];
for email in emails {
let anonymized_email = email.anonymize_email(); // This is the API
that Santa wants!
println!("Original: {} -> Anonymized: {}", email, anonymized_email);
}
}