# faml

[![version](https://img.shields.io/badge/dynamic/toml?url=https%3A%2F%2Fraw.githubusercontent.com%2Ffawdlstty%2Ffaml%2Fmain%2Ffaml%2FCargo.toml&query=package.version&label=version)](https://crates.io/crates/faml)
![status](https://img.shields.io/github/actions/workflow/status/fawdlstty/faml/rust.yml)

[English](README.md) | 简体中文

Faml 是一款动态配置脚本语言，可在配置文件里嵌入脚本代码，实现动态更新配置。

示例代码：

```faml
[hello]
value = 30
name = $"hello {value + 12}" // hello 42
```

# 文档

<https://faml.fawdlstty.com/>

<!--

## 用户手册

安装：在项目目录下运行 `cargo add faml`

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

下载并编译静态库（或动态库）

```shell
git clone git@github.com:fawdlstty/faml.git
cd faml
# 构建支持C API的版本
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
```

此时静态库（或动态库）位于 `target/release` 目录下。将其拷贝至C++项目，并引用

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
    auto oeroot = faml::FamlExpr::from_str(R"(
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

执行命令:
```sh
dotnet add package faml
```

示例:
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

### 其他功能

当满足条件时值可用：

```faml
[hello]

value = 12

@if value == 12
name = $"hello {value}"
```

-->
