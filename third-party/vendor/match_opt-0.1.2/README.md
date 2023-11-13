##  match_opt ![Build Status](https://github.com/huitseeker/match-opt/workflows/Rust/badge.svg)

**match_opt** is a macro that converts a partial pattern-match into one that returns an `Option`.
This can be used as a shorthand to simplify a complex pattern-match, for which only one or few cases are useful.

For example:

```rust
let some_results = vec!["1", "2", "3", "foo", "bar", "4"].map(|v| v.parse::<usize>());
let some_pattern_match = some_results.filter_map{|x| match x {
    Ok(v) if v > 2 => Some(v),
    _ => None,
}};
// this is equivalent to
let alternate_pattern_match = some_results.filter_map{|x| match_opt!(x, OK(v) if v > 2 => v)};
```

This crate has no dependencies.


Documentation
-------------

The API can be found [here](https://docs.rs/match-opt/).

Installation
------------

Add the following line to the dependencies of your `Cargo.toml`:

```toml
match_opt = "0.1.0"
```

Contributors
------------

The authors of this code is François Garillot ([@huitseeker](https://github.com/huitseeker)).

License
-------

This project is [MIT licensed](./LICENSE).
