# faml

[![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Ffaml%2Fmain%2Ffaml%2FCargo.toml&query=package.version&label=version)](https://crates.io/crates/faml)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/faml/rust.yml)
[![docs](https://img.shields.io/badge/docs-latest-blue)](https://faml.fawdlstty.com/en/)

English | [简体中文](README.zh_CN.md)

**faml** is a minimalist and powerful dynamic configuration language that extends TOML with scripting capabilities, enabling dynamic configuration updates at runtime.

## Key Features

- **TOML-Compatible Syntax**: Familiar syntax for TOML users with enhanced dynamic capabilities
- **Dynamic Expressions**: Embed expressions using `$"..."` for runtime value computation
- **Conditional Configuration**: Use `@if` directives to conditionally enable configuration blocks
- **Rich Data Types**: Built-in support for durations, distances, and quantified numbers
- **Cross-Language Support**: Native Rust API with C/C++ and C# bindings
- **Runtime Mutability**: Modify configuration values at runtime and automatically update dependent values

## Quick Example

```faml
[server]
port = 8080
host = "localhost"
connection_string = $"{host}:{port}/api"

[cache]
ttl = 5 minutes
max_size = 100 MB
```

## Installation

### Rust

```bash
cargo add faml
```

### C++

```bash
git clone https://github.com/fawdlstty/faml.git
cd faml
cargo build --release
```

### C#

```bash
dotnet add package faml
```

## Usage

### Rust

```rust
use faml::FamlExpr;

fn main() -> anyhow::Result<()> {
    let config_str = r#"
[server]
port = 8080
host = "localhost"
connection_string = $"{host}:{port}/api"
"#;
    
    let mut config = FamlExpr::from_str(config_str)?;
    config["server"]["port"].set_int(9000);  // Dynamic update
    
    let connection_string = config["server"]["connection_string"].evaluate()?.as_str();
    println!("{}", connection_string); // localhost:9000/api
    Ok(())
}
```

### C++

```cpp
#include "faml/faml.hpp"

int main() {
    auto oexpr = faml::FamlExpr::from_str(R"(
[server]
port = 8080
host = "localhost"
connection_string = $"{host}:{port}/api"
)");
    
    if (oexpr.index() == 1) {
        std::cout << std::get<std::string>(oexpr) << std::endl;
        return 0;
    }
    
    auto expr = std::get<faml::FamlExpr>(oexpr);
    expr["server"]["port"].set_int(9000);  // Dynamic update
    
    auto ovalue = expr.evaluate();
    if (ovalue.index() == 1) {
        std::cout << std::get<std::string>(ovalue) << std::endl;
        return 0;
    }
    
    auto value = std::get<faml::FamlValue>(ovalue);
    std::cout << value["server"]["connection_string"].as_str() << std::endl; // localhost:9000/api
    return 0;
}
```

### C#

```csharp
using faml;

string configStr = @"
[server]
port = 8080
host = ""localhost""
connection_string = $""{host}:{port}/api""
";

var expr = FamlExpr.from_str(configStr);
expr["server"]["port"].set_int(9000);  // Dynamic update

var value = expr.evaluate();
Console.WriteLine(value["server"]["connection_string"].as_str()); // localhost:9000/api
```

## Documentation

- [Introduction](https://faml.fawdlstty.com/en/guide/00_introduction.html)
- [Getting Started](https://faml.fawdlstty.com/en/guide/01_hello_world.html)
- [Syntax Guide](https://faml.fawdlstty.com/en/guide/02_structs_and_types.html)
- [Expressions](https://faml.fawdlstty.com/en/guide/03_expressions.html)
- [Methods](https://faml.fawdlstty.com/en/guide/04_methods.html)

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.
