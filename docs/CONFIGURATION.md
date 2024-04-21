# Configuring johnnybgoode

## configuration.yaml

### Mandatory values

- `johnnydecimal_home`: The path to the top folder of your JohnnyDecimal tree
- `name_scheme`: either ACID or DACID. "ACID" or "Area, Category, ID" is the official way that JohnnyDecimal suggests for numbering. In practice it looks like 26.94 or 53.08. "DACID" or "Domain, Area, Category, ID" is another option that features a 1-letter domain identifier. This is to be used when there are multiple JohnnyDecimal systems running in parallel, as a means of avoiding confusion.
