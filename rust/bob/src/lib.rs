pub fn reply(message: &str) -> &str {
    enum Addressal {
        Question,
        Yell,
        QuestionYell,
        Nothing,
        Alphabetic,
        Numeric,
    }

    let mut response = Addressal::Nothing;
    for c in message.chars() {
        if c == '?' {
            response = match response {
                Addressal::Yell => Addressal::QuestionYell,
                _ => Addressal::Question,
            };
        } else if c.is_uppercase() || c == '!' {
            response = match response {
                Addressal::Nothing | Addressal::Numeric => Addressal::Yell,
                _ => response,
            };
        } else if c.is_ascii_alphabetic() {
            response = Addressal::Alphabetic;
        } else if c.is_ascii_digit() {
            response = Addressal::Numeric;
        }
    }

    match response {
        Addressal::Question => "Sure.",
        Addressal::Yell => "Whoa, chill out!",
        Addressal::QuestionYell => "Calm down, I know what I'm doing!",
        Addressal::Nothing => "Fine. Be that way!",
        _ => "Whatever.",
    }
}
