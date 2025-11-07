# faml

[![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Ffaml%2Fmain%2Ffaml%2FCargo.toml&query=package.version&label=version)](https://crates.io/crates/faml)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/faml/rust.yml)

English | [简体中文](README.zh_CN.md)

Faml is a dynamic configuration scripting language that can embed script code in the configuration file to achieve dynamic configuration update.

example code:

```faml
[hello]
value = 30
name = $"hello {value + 12}" // hello 42
```

# Document

<https://faml.fawdlstty.com/en/>

<!--

## Manual

Install: Run `cargo add faml` in the project directory

```rust
fn main() -> anyhow::Result<()> {
    let faml_str = r#"
[hello]
value = 12
name = $"hello {value + 12}"
"#;
    let mut root = faml::FamlExpr::from_str(faml_str)?;
    root["hello"]["value"].set_int(30);
    println!("{}", root["hello"]["name"].evaluate()?.as_str()); // hello 42
    Ok(())
}
```

### C++

Download and compile static libraries (or dynamic libraries)

```shell
git clone git@github.com:fawdlstty/faml.git
cd faml
# Build with C API support
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
```

The static library (or dynamic library) is generated in the `target/release` directory. Copy it to the C++ project and reference it

```cpp
#include <iostream>
#include <string>

#include "faml/faml.hpp"
#ifdef _MSC_VER
#pragma comment(lib, "ws2_32.lib")
#pragma comment(lib, "ntdll.lib")
#pragma comment(lib, "bcrypt.lib")
#pragma comment(lib, "Userenv.lib")
#pragma comment(lib, "faml.lib")
#endif

int main() {
    auto oexpr = faml::FamlExpr::from_str(R"(
[hello]
value = 12
name = $"hello {value + 12}"
)");
    if (oeroot.index() == 1) {
        std::cout << std::get<std::string>(oeroot) << std::endl;
        return 0;
    }
    auto eroot = std::get<faml::FamlExpr>(oeroot);
    eroot["hello"]["value"].set_int(30);
    auto oroot = eroot.evaluate();
    if (oroot.index() == 1) {
        std::cout << std::get<std::string>(oroot) << std::endl;
        return 0;
    }
    auto root = std::get<faml::FamlValue>(oroot);
    std::cout << root["hello"]["name"].as_str() << std::endl; // hello 42
    return 0;
}
```

### C#

Run command:
```sh
dotnet add package faml
```

Example:
```csharp
using System;

namespace test {
    public class Program {
        public static void Main () {
            string src = """
[hello]
value = 12
name = $"hello {value + 12}"
""";
            var eroot = faml.FamlExpr.from_str (src);
            eroot ["hello"] ["value"].set_int (30);
            var root = eroot.evaluate ();
            Console.WriteLine (root ["hello"] ["name"].as_str()); // hello 42
            Console.ReadKey ();
        }
    }
}
```

### Other features

The value is available when the conditions are met:

```faml
[hello]

value = 12

@if value == 12
name = $"hello {value}"
```

TODO: 重量、电流、电压、温度等单位

-->
