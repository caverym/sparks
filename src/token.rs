use std::fmt::Formatter;

pub enum Token<> {
    Bot(String),
    User(String),
    None,
}

impl Token {
    pub fn new<T: ToString>(account_kind: &str, token: T) -> Token {
        match account_kind {
            "bot" => Token::Bot(token.to_string()),
            "user" => Token::User(token.to_string()),
            _ => Token::None,
        }
    }
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let token = match self {
            Self::Bot(t) => t,
            Self::User(t) => t,
            _ => "",
        };

        write!(f, "{}", token)
    }
}
