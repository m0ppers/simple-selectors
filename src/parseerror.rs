#[derive(Debug)]
pub enum ParseError {
    EmptySelector,
    InvalidKey(usize),
    InvalidOperator(usize),
    ExpectingEndOrComma(usize),
    ExpectingValue(usize),
    ExpectingLeftParenthesis(usize),
}
