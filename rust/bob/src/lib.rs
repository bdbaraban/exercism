pub fn reply(message: &str) -> &str {
    let message = message.trim_matches(char::is_whitespace);

    if message.is_empty() {
        return "Fine. Be that way!";
    }
    if message.chars().all(|c| !c.is_lowercase()) && message.chars().any(|c| c.is_alphabetic()) {
        if message.ends_with('?') {
            return "Calm down, I know what I'm doing!";
        }
        return "Whoa, chill out!";
    }
    if message.ends_with('?') {
        return "Sure.";
    }
    "Whatever."
}
