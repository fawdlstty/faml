# faml

[![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Ffaml%2Fmain%2Ffaml%2FCargo.toml&query=package.version&label=version)](https://crates.io/crates/faml)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/faml/rust.yml)
[![docs](https://img.shields.io/badge/docs-latest-blue)](https://faml.fawdlstty.com/)

[English](README.md) | 简体中文

**faml** 是一款极简而强大的动态配置语言，它扩展了 TOML 语法，增加了脚本能力，支持在运行时动态更新配置。

## 核心特性

- **TOML 兼容语法**：对熟悉 TOML 的用户友好，同时增强动态能力
- **动态表达式**：使用 `$"..."` 语法嵌入表达式，实现运行时值计算
- **条件配置**：通过 `@if` 指令有条件地启用配置块
- **丰富的数据类型**：内置支持持续时间、距离和量化数字等特殊类型
- **多语言支持**：提供 Rust 原生 API 以及 C/C++ 和 C# 绑定
- **运行时可变性**：支持在运行时修改配置值并自动更新依赖项

## 快速示例

```faml
[server]
port = 8080
host = "localhost"
connection_string = $"{host}:{port}/api"

[cache]
ttl = 5 minutes
max_size = 100 MB
```

## 安装方式

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

## 使用方法

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
    config["server"]["port"].set_int(9000);  // 动态更新
    
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
    expr["server"]["port"].set_int(9000);  // 动态更新
    
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
expr["server"]["port"].set_int(9000);  // 动态更新

var value = expr.evaluate();
Console.WriteLine(value["server"]["connection_string"].as_str()); // localhost:9000/api
```

## 文档资源

- [前言](https://faml.fawdlstty.com/guide/00_introduction.html)
- [快速开始](https://faml.fawdlstty.com/guide/01_hello_world.html)
- [语法指南](https://faml.fawdlstty.com/guide/02_structs_and_types.html)
- [表达式](https://faml.fawdlstty.com/guide/03_expressions.html)
- [方法](https://faml.fawdlstty.com/guide/04_methods.html)

## 许可证

本项目采用 MIT 许可证，详情请见 [LICENSE](LICENSE) 文件。
