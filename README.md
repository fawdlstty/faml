# faml

[![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Ffaml%2Fmain%2Ffaml%2FCargo.toml&query=package.version&label=version)](https://crates.io/crates/faml)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/faml/rust.yml)
[![docs](https://img.shields.io/badge/docs-latest-blue)](https://faml.fawdlstty.com/en/)

English | [简体中文](README.zh_CN.md)

**faml** is a minimalist and powerful dynamic configuration language that extends TOML with scripting capabilities, enabling dynamic configuration updates at runtime.

## Help

Having trouble? Join the QQ group for help: `1018390466`

## Key Features

- **TOML-Compatible Syntax**: Familiar syntax for TOML users with enhanced dynamic capabilities
- **Dynamic Expressions**: Embed expressions using `$"..."` for runtime value computation
- **Conditional Configuration**: Use `@if` directives to conditionally enable configuration blocks
- **Rich Data Types**: Built-in support for durations, distances, and quantified numbers
- **Cross-Language Support**: Native Rust API with C/C++ and C# bindings
- **Runtime Mutability**: Modify configuration values at runtime and automatically update dependent values

## Documentation

- [Introduction](https://faml.fawdlstty.com/en/guide/00_introduction.html)
- [Getting Started](https://faml.fawdlstty.com/en/guide/01_hello_world.html)
- [Syntax Guide](https://faml.fawdlstty.com/en/guide/02_structs_and_types.html)
- [Expressions](https://faml.fawdlstty.com/en/guide/03_expressions.html)
- [Methods](https://faml.fawdlstty.com/en/guide/04_methods.html)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

<!--
Plan:

1. i18n support
2. compiler
-->
