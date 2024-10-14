Notes regarding error codes and such

Even numbered error codes are recoverable. Odd numbered ones are not.

## 1000-series, parsing related.
JBG-1001 - Regex unable to find any matching capture groups, inside of src/lib.rs -> extract_location()

JBG-1028 - Regex in config cannot be parsed

## 3000-series, nonexistence related.
JBG-3077 - Location key passed to src/lib.rs -> get_path() that does not exist in the hashmap built my scan_to_map.
