use itertools::Itertools;
use std::collections::HashMap;
use std::str::CharIndices;
use std::iter::Peekable;

use super::ParseError;

pub type LabelMap<'a> = HashMap<&'a str, &'a str>;

fn parse_key(index: &mut usize, chars: &mut Peekable<CharIndices>) -> Result<String, ParseError> {
    // find the first non matching char
    let key: String = chars
        .take_while_ref(|&(_i, c)| match c {
            'a'...'z' => true,
            'A'...'Z' => true,
            '0'...'9' => true,
            '-' => true,
            '_' => true,
            _ => false,
        })
        .map(|(i, c)| {
            *index = i;
            c
        })
        .collect();

    if key.len() == 0 {
        let next = chars.peek();
        if let Some(&(i, c)) = next {
            *index = i;
            return match c {
                ',' => Ok(key),
                _ => Err(ParseError::InvalidKey(i)),
            };
        } else {
            return Err(ParseError::InvalidKey(*index));
        }
    }
    Ok(key)
}

fn skip_whitespaces(index: &mut usize, chars: &mut Peekable<CharIndices>) {
    // skip until no more whitespaces
    while let Some(&(i, c)) = chars.peek() {
        *index = i;
        if c == ' ' {
            chars.next();
        } else {
            break;
        }
    }
}

#[derive(Debug, PartialEq)]
enum Operator {
    Equality,
    InEquality,
    InSet,
    NotInSet,
}

fn parse_operator(
    index: &mut usize,
    chars: &mut Peekable<CharIndices>,
) -> Result<Operator, ParseError> {
    match chars.next() {
        Some((i, c)) => {
            *index = i;
            match c {
                '=' => {
                    if let Some(&(i, c)) = chars.peek() {
                        if c == '=' {
                            chars.next();
                            *index = i;
                        }
                    }
                    Ok(Operator::Equality)
                }
                '!' => {
                    match chars.next() {
                        Some((i, c)) => {
                            *index = i;
                            match c {
                                '=' => Ok(Operator::InEquality),
                                _ => Err(ParseError::InvalidOperator(i)),
                            }
                        }
                        None => Err(ParseError::InvalidOperator(i)),
                    }
                }
                'i' => {
                    match chars.next() {
                        Some((i, c)) => {
                            *index = i;
                            match c {
                                'n' => Ok(Operator::InSet),
                                _ => Err(ParseError::InvalidOperator(i)),
                            }
                        }
                        None => Err(ParseError::InvalidOperator(i)),
                    }
                }
                'n' => {
                    let s: String = chars.take(4).map(|(_i, c)| c).collect();

                    if s == "otin" {
                        *index += 5;
                        Ok(Operator::NotInSet)
                    } else {
                        Err(ParseError::InvalidOperator(i))
                    }
                }
                _ => Err(ParseError::InvalidOperator(i)),
            }
        }
        None => Err(ParseError::InvalidOperator(*index)),
    }
}

fn parse_value(index: &mut usize, chars: &mut Peekable<CharIndices>) -> Result<String, ParseError> {
    let value: String = chars
        .take_while_ref(|&(_i, c)| match c {
            '!' => false,
            '=' => false,
            ',' => false,
            '(' => false,
            ')' => false,
            _ => !c.is_whitespace(),
        })
        .map(|(i, c)| {
            *index = i;
            c
        })
        .collect();

    if value.len() == 0 {
        Err(ParseError::ExpectingValue(*index))
    } else {
        Ok(value)
    }
}

fn parse_set(
    index: &mut usize,
    chars: &mut Peekable<CharIndices>,
) -> Result<Vec<String>, ParseError> {
    let mut result = vec![];

    let parens = chars.next();
    match parens {
        None => Err(ParseError::ExpectingLeftParenthesis(*index)),
        Some((i, c)) => {
            *index = i;
            match c {
                '(' => Ok(()),
                _ => Err(ParseError::ExpectingLeftParenthesis(i)),
            }
        }
    }?;

    let mut first = true;
    loop {
        skip_whitespaces(index, chars);
        if !first {
            match chars.next() {
                None => return Err(ParseError::ExpectingEndOrComma(*index)),
                Some((i, c)) => {
                    *index = i;
                    match c {
                        ')' => {
                            break;
                        }
                        ',' => (),
                        _ => {
                            return Err(ParseError::ExpectingEndOrComma(i));
                        }
                    }
                }
            }
        }
        first = false;
        result.push(parse_value(index, chars)?);
    }
    Ok(result)
}

fn compare_value(label: Option<&&str>, value: String) -> bool {
    match label {
        None => false,
        Some(label) => *label == value,
    }
}

fn compare_set(label: Option<&&str>, value: Vec<String>) -> bool {
    match label {
        None => false,
        Some(label) => value.contains(&String::from(*label)),
    }
}

fn parse_operation(
    index: &mut usize,
    label: Option<&&str>,
    chars: &mut Peekable<CharIndices>,
) -> Result<bool, ParseError> {
    let operator = parse_operator(index, chars)?;
    skip_whitespaces(index, chars);

    Ok(match operator {
        Operator::Equality => compare_value(label, parse_value(index, chars)?),
        Operator::InEquality => !compare_value(label, parse_value(index, chars)?),
        Operator::InSet => compare_set(label, parse_set(index, chars)?),
        Operator::NotInSet => !compare_set(label, parse_set(index, chars)?),
    })
}

fn parse_requirement(
    index: &mut usize,
    chars: &mut Peekable<CharIndices>,
    labels: &LabelMap,
) -> Result<bool, ParseError> {
    skip_whitespaces(index, chars);

    let mut positive = true;
    if let Some(&(i, c)) = chars.peek() {
        if c == '!' {
            *index = i;
            positive = false;
            chars.next();
            skip_whitespaces(index, chars);
        }
    }
    let key = parse_key(index, chars)?;
    skip_whitespaces(index, chars);

    let simple_exists_check = {
        let next = chars.peek();
        match next {
            None => true,
            Some(&(i, c)) => {
                *index = i;
                match c {
                    ',' => true,
                    _ => false,
                }
            }
        }
    };
    let value = match simple_exists_check {
        true => labels.contains_key(key.as_str()),
        false => parse_operation(index, labels.get(key.as_str()), chars)?,
    };

    Ok(if positive { value } else { !value })
}

fn parse_selector(selector: &str, labels: &LabelMap) -> Result<bool, ParseError> {
    let mut chars = selector.char_indices().peekable();

    let mut index = 0;
    let mut result = parse_requirement(&mut index, &mut chars, labels)?;
    loop {
        skip_whitespaces(&mut index, &mut chars);
        if let Some(&(i, c)) = chars.peek() {
            if c == ',' {
                chars.next();
            }
            index = i;
        } else {
            break;
        }
        result = parse_requirement(&mut index, &mut chars, labels)? && result;

    }
    Ok(result)
}

pub fn parse(selector: &str, labelmap: &LabelMap) -> Result<bool, ParseError> {
    if selector.len() == 0 {
        return Err(ParseError::EmptySelector);
    }
    parse_selector(selector, labelmap)
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_should_properly_parse_a_full_key() {
        let selector = "test";

        let mut index = 0;
        let mut char_indices = selector.char_indices().peekable();
        let result = super::parse_key(&mut index, &mut char_indices);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");
    }

    #[test]
    fn it_will_parse_keys_until_invalid_chars() {
        let selector = "test/";

        let mut index = 0;
        let mut char_indices = selector.char_indices().peekable();
        let result = super::parse_key(&mut index, &mut char_indices);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "test");

        let result = super::parse_key(&mut index, &mut char_indices);
        assert_matches!(result, Err(super::ParseError::InvalidKey(4)));
    }

    #[test]
    fn it_will_accept_valid_keys_but_stop_consuming() {
        let selector = "test /";

        let mut index = 0;
        let mut char_indices = selector.char_indices().peekable();
        let key = super::parse_key(&mut index, &mut char_indices);
        assert_eq!(key.unwrap(), "test");
        assert_matches!(char_indices.next(), Some((4, ' ')));
    }

    #[test]
    fn it_will_parse_single_char_equality() {
        let op = "=";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), super::Operator::Equality);
    }

    #[test]
    fn it_will_parse_double_char_equality() {
        let op = "==";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), super::Operator::Equality);

        assert!(char_indices.next().is_none());
    }

    #[test]
    fn it_will_reject_garbage() {
        let op = "#";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert_matches!(result, Err(super::ParseError::InvalidOperator(0)));
    }

    #[test]
    fn it_will_parse_inequality() {
        let op = "!=";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), super::Operator::InEquality);

        assert!(char_indices.next().is_none());
    }

    #[test]
    fn it_will_reject_garbage_inequality() {
        let op = "!#";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert!(result.is_err());
        assert_matches!(result, Err(super::ParseError::InvalidOperator(1)));
    }

    #[test]
    fn it_will_accept_in() {
        let op = "in";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), super::Operator::InSet);
    }

    #[test]
    fn it_will_accept_notin() {
        let op = "notin";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), super::Operator::NotInSet);
    }

    #[test]
    fn it_will_refuse_garbage_in() {
        let op = "ian";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert_matches!(result, Err(super::ParseError::InvalidOperator(1)));
    }

    #[test]
    fn it_will_refuse_garbage_notin() {
        let op = "no";

        let mut index = 0;
        let mut char_indices = op.char_indices().peekable();
        let result = super::parse_operator(&mut index, &mut char_indices);
        assert_matches!(result, Err(super::ParseError::InvalidOperator(0)));
    }
}
