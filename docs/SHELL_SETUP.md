# Setting up the `johnny jump` command in your shell

Numerous shells exist, so as scripts are created for each, they will be added here. The list is in alphabetical order.

## Bash/Zsh

Open your `.bashrc` or `.zshrc` in a text editor of your choice. Add the following:

```bash
function jump() {
  cd $(johnnybgoode path $1)
}
```

## Nushell

Open your config with `vim $nu.config-path` (or another editor of your choice) and scroll to the bottom and append the following code:

```nu
def --env jump [code] {
    cd (johnnybgoode path $code)
}
```
