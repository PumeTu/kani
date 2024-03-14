enum TokenType {
    ILLEGAL, // token/character we don't know about
    EOF,
    // Identifiers + Literals
    IDENT,
    INT,
    // Operators
    ASSIGN,
    PLUS,
    // Delimiters
    COMMA,
    SEMICOLON,
    LPAREN,
    RPAREN,
    LBRACE,
    RRACE,
    // Keywords
    FUNCTION,
    LET,
}

struct Token {
    token: TokenType,
    literal: String,
}

struct Lexer {
    input: String,
    position: usize,      // current position in input (points to current_char)
    read_position: usize, // current reading position in input (after current_char)
    current_char: char,   // current char under examnitation
}

impl Lexer {
    fn new(input: String) -> Self {
        Self { input }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_next_token() {
        let input = "=+(){},";
    }
}
