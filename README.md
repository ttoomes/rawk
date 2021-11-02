# rawk

*Kind of like awk...but not.*

## Description

`rawk` performs a naive deduplication of a list of employees provided as a `csv` based on the filter provided. The employees can be deduplicated based on a matching email, phone number, or email or phone number.

rawk is inspired by [awk](https://en.wikipedia.org/wiki/AWK).

Accepted filters are:

* `email`
* `phone`
* `email_or_phone`

```bash
rawk email input.csv  output.csv
```

## Testing

To test, run the following command:

```bash
cargo test
```

## Building

To build `rawk` using `cargo`, run the following command on your target platform to build a release.

```bash
cargo build --release
```

## To Do
- [ ] Improve test coverage
- [ ] Allow output to stdout to mimic awk