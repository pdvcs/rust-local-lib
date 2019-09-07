pub fn greet(locale: &str) -> &'static str {
    match locale {
        "en" => "Hello",
        "fr" => "Bonjour",
        "de" => "Hallo",
        "dk" => "Hej",
        _ => "Greetings",
    }
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn test_greet() {
        assert_eq!("Bonjour", greet("fr"));
        assert_eq!("Hej", greet("dk"));
        assert_eq!("Hallo", greet("de"));
        assert_eq!("Hello", greet("en"));
        assert_eq!("Greetings", greet("is"));
    }
}