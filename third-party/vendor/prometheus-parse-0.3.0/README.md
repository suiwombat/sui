# prometheus-parse

Simple but effective Rust parser for the Prometheus scrape format.

```rust
let body = reqwest::get("https://prometheus.example.com/metrics")?
    .text()?;
let lines: Vec<_> = body.lines().map(|s| Ok(s.to_owned())).collect();

let metrics = prometheus_parse::Scrape::parse(lines.into_iter())?;
```

### Attribution

This crate is 99.99% lifted from [prometheus-scrape](https://crates.io/crates/prometheus-scrape) with some minor API changes and a GitHub repo to encourage PRs.
It was again forked from [prometheus-parse-rs](https://github.com/ccakes/prometheus-parse-rs) due to inactivity for PR merges.

## License

Apache License 2.0 - same as original project, and unchanged since.