/// Bob response generator. Responds:
///   "Sure." if questioned.
///   "Whoa, chill out!" if yelled at.
///   "Calm down, I know what I'm doing!" if yelled a question at.
///   "Fine. Be that way!" if addressed without saying anything.
///   "Whatever." to anything else.
pub fn reply(message: &str) -> &str {
    let message = message.trim();

    if message.is_empty() {
        "Fine. Be that way!"
    } else if !message.contains(char::is_lowercase) && message.contains(char::is_alphabetic) {
        if message.ends_with('?') {
            "Calm down, I know what I'm doing!"
        } else {
            "Whoa, chill out!"
        }
    } else if message.ends_with('?') {
        "Sure."
    } else {
        "Whatever."
    }
}
