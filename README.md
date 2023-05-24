# dprint-plugin-sexpr

Format S-expressions in `.scm` files using
[rsexpr](https://crates.io/crates/rsexpr).

## Install

Add the plugin to your config file by running
`dprint config add RubixDev/sexpr`.

Don't forget to add `scm` to your `includes` pattern.

## Configuration

| Name        | Type | Default              | Possible values |
| ----------- | ---- | -------------------- | --------------- |
| indentWidth | u8   | global config or `2` | `0` — `255`     |
