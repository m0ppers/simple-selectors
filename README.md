# simple-selectors

[![Build Status](https://www.travis-ci.org/m0ppers/simple-selectors.svg?branch=master)](https://www.travis-ci.org/m0ppers/simple-selectors) [![License: Unlicense](https://img.shields.io/badge/license-Unlicense-blue.svg)](http://unlicense.org/)

k8s style selectors for rust.

## BNF
```
  <selector-syntax>         ::= <requirement> | <requirement> "," <selector-syntax>
  <requirement>             ::= [!] KEY [ <set-based-restriction> | <exact-match-restriction> ]
  <set-based-restriction>   ::= "" | <inclusion-exclusion> <value-set>
  <inclusion-exclusion>     ::= <inclusion> | <exclusion>
  <exclusion>               ::= "notin"
  <inclusion>               ::= "in"
  <value-set>               ::= "(" <values> ")"
  <values>                  ::= VALUE | VALUE "," <values>
  <exact-match-restriction> ::= ["="|"=="|"!="] VALUE
```

*) This section has been copy pasted from https://github.com/blendlabs/go-selector (this is what this library is based upon)

## Usage

```rust
let mut labels = LabelMap::new();
labels.insert("test", "test");
labels.insert("test1", "test");
let result = parse("test = test, test1 in (test)", &labels);
```