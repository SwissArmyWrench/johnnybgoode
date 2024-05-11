# Configuring johnnybgoode

## configuration.yaml

### Mandatory values

- `johnnydecimal_home`: The path to the top folder of your JohnnyDecimal tree

### Optional values

- `regex`: Specify a custom regex pattern to match on to find the "ACID" numbers. It *must* include two named capture groups (AC and ID) to each return two digits. It must be compatible with Rust's regex crate, which is documented [here](https://docs.rs/regex/latest/regex/index.html). The default regex is `(?<AC>[0-9]{2})[ \.]?(?<ID>[0-9]{2})` which matches on numbers formatted in the following ways: 12.34, 12 34, or 1234.
