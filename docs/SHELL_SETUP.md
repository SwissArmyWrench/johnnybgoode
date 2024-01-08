# Setting up the `johnny jump` command in your shell

Numerous shells exist, so as scripts are created for each, they will be added here. The list is in alphabetical order.

## Nushell

Open your config with `nano $nu.config-path` (or another editor of your choice) and scroll to the bottom and append the following code:

```nu
def --env "johnny jump" [code] {
    cd (johnny path $code)
}
```
