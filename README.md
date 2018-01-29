# simple-selectors

[![Build Status](https://www.travis-ci.org/m0ppers/simple-selectors.svg?branch=master)](https://www.travis-ci.org/m0ppers/simple-selectors)

k8s style selectors for rust:

```rust
let mut labels = LabelMap::new();
labels.insert("test", "test");
labels.insert("test1", "test");
let result = parse("test = test, test1 in (test)", &labels);
```