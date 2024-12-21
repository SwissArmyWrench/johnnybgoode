Notes regarding error codes and such

Even numbered error codes are recoverable. Odd numbered ones are not.

## 1000-series, parsing related.
JBG-1001 - Regex unable to find any matching capture groups, inside of src/lib.rs -> extract_location()

JBG-1028 - Regex in config cannot be parsed. Not that this check passing does not guarantee the regex will be useful. It is possible to pass a valid regex that will not match any files in your system. This error simply is thrown when the supplied regex cannot be compiled. In this case, johnnybgoode discards the supplied one and uses its built-in default, which can match on folders with numbers formatted as "12.34", "1234", or "12 34".

## 3000-series, nonexistence related.
JBG-3077 - Location key passed to src/lib.rs -> get_path() that does not exist in the hashmap built my scan_to_map.

## 4000-series, configuation related
JBG-4289 - Configuration file specified in environment variable does not exist or cannot be accessed
JBG-4293 - Configuration in default location does not exist or cannot be accessed
JBG-4271 - Johnnydecimal root folder does not exist or cannot be accessed
