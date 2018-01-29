#[macro_use]
extern crate assert_matches;
extern crate simple_selectors;

use simple_selectors::*;

#[test]
fn it_will_fail_for_empty_selectors() {
    let labels = LabelMap::new();
    assert_matches!(parse("", &labels), Err(ParseError::EmptySelector));
}

#[test]
fn it_will_return_false_for_unknown_labels() {
    let labels = LabelMap::new();
    let result = parse("testii", &labels);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn it_will_return_true_for_known_labels() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    let result = parse("test", &labels);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn it_will_ignore_whitespaces_in_the_beginning() {
    let selector = " hmpf";

    let labels = LabelMap::new();
    let result = parse(selector, &labels);

    assert_eq!(result.unwrap(), false);
}

#[test]
fn it_will_combine_requirements_with_and_positive() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    labels.insert("test1", "test");
    let result = parse("test,test1", &labels);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn it_will_combine_requirements_with_and_negative() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    labels.insert("test1", "test");
    let result = parse("test,test2", &labels);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn it_will_negate_things_properly() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    let result = parse("!test", &labels);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn it_will_negate_things_properly_with_whitespaces() {
    let mut labels = LabelMap::new();
    labels.insert("test1", "test");
    let result = parse(" ! test", &labels);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn it_will_check_equality() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    let result = parse("test = test", &labels);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn it_will_check_inequality() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    let result = parse("test != test", &labels);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}

#[test]
fn it_will_check_sets() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    let result = parse("test in (test)", &labels);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn it_will_check_sets_with_ws() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    let result = parse("test in ( test ) ", &labels);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn it_will_check_not_sets() {
    let mut labels = LabelMap::new();
    labels.insert("test", "test");
    let result = parse("test notin (test)", &labels);
    assert!(result.is_ok());
    assert!(!result.unwrap());
}
