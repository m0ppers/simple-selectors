#[derive(Debug)]
pub enum ParseError {
    /// a selector must not be empty
    EmptySelector,
    /// the key at the position was invalid (i.e. some chars are not allowed within a key)
    InvalidKey(usize),
    /// the operator at the position is invalid
    InvalidOperator(usize),
    /// the selector must either finish at the position or another expression should start separated by a comma
    ExpectingEndOrComma(usize),
    /// a value was expected at the position
    ExpectingValue(usize),
    /// expecting opening parenthesis
    ExpectingLeftParenthesis(usize),
}
