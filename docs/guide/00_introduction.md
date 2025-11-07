# 前言

TOML 是一种简洁的配置语言，具有良好的可读性和易用性。它支持注释和嵌套结构，语法直观，特别适合作为配置文件格式。与 JSON 和 YAML 相比，TOML 在手工编辑时更不容易出错。

作为一种静态配置语言，TOML 在处理动态配置需求时存在局限性。例如在 Rust 项目的 Cargo.toml 中，条件依赖通常通过字符串形式表示：

```toml
[dependencies]
[target.'cfg(not(target_env = "msvc"))'.dependencies]
tikv-jemallocator = { version = "0.6.0", features = ["profiling", "unprefixed_malloc_on_supported_platforms"] }
```

在这种情况下，`cfg(not(target_env = "msvc"))` 被作为字符串处理，由构建工具解析为条件表达式。这种做法可能导致以下问题：
1. 混合不同语言语法增加了配置复杂性
2. 嵌入的表达式缺乏明确的语法规范

针对动态配置需求，业界已有一些解决方案：

**KCL**

由 CNCF 托管的动态配置语言，示例：

```kcl
apiVersion = "apps/v1"
kind = "Deployment"
metadata = {
    name = "nginx"
    labels.app = name
}
spec = {
    replicas = 3
    selector.matchLabels = metadata.labels
    template.metadata.labels = metadata.labels
    template.spec.containers = [
        {
            name = metadata.name
            image = "${metadata.name}:1.14.2"
            ports = [{ containerPort = 80 }]
        }
    ]
}
```

**PKL**

由苹果公司开发的动态配置语言，示例：

```pkl
name = "Swallow"

job {
  title = "Sr. Nest Maker"
  company = "Nests R Us"
  yearsOfExperience = 2
}
```

现有的动态配置语言通常采用类似 JSON 的结构化组织方式，这与 TOML 的风格有所不同。此外，每种语言都有其特定的语法规则，需要额外的学习成本。

## FAML 的核心优势

FAML 旨在结合 TOML 的简洁语法与动态配置能力，提供一种既保持 TOML 易用性又具备动态特性的配置语言选择。与其他动态配置语言相比，FAML 具有以下独特优势：

### 1. 语法简洁，易于上手

FAML 延续了 TOML 的简洁语法风格，使用方括号定义配置块，使用等号赋值，保持了与 TOML 一致的可读性。对于熟悉 TOML 的开发者来说，学习 FAML 几乎没有门槛。

```faml
[server]
port = 8080
host = "localhost"
```

相比之下，KCL 使用大括号 `{}` 定义对象，更接近 JSON 的语法；PKL 则采用了类似结构体的定义方式。FAML 的 TOML 风格语法使其在可读性和易写性方面具有优势，特别是在手工编辑配置文件时更为友好。

### 2. 强大的动态表达式支持

FAML 支持在配置中嵌入表达式，可以引用其他配置项、进行算术运算、逻辑判断等操作，实现真正的动态配置。

```faml
[database]
host = "localhost"
port = 5432
connection_string = $"postgresql://{host}:{port}/mydb"
```

与 TOML 相比，FAML 的表达式功能更为强大。TOML 作为静态配置语言不支持表达式计算，而 FAML 的 `$""` 语法允许在字符串中嵌入表达式，实现动态内容生成。KCL 和 PKL 也都支持类似的表达式功能，但 FAML 的语法更加简洁明了。

### 3. 条件配置控制

通过 `@if` 指令，FAML 可以根据条件控制配置项是否生效，这在不同环境下的配置管理中非常有用。

```faml
[app]
env = "production"

@if env == "development"
log_level = "debug"

@if env == "production"
log_level = "error"
```

这一特性使 FAML 在条件配置方面优于标准的 TOML，因为 TOML 本身不支持条件逻辑。虽然 KCL 和 PKL 也可以实现条件配置，但 FAML 的 `@if` 语法更加直观，更符合配置文件的使用习惯。

### 4. 内置特殊数据类型

FAML 内置了对持续时间、距离等特殊数据类型的支持，可以直接在配置中使用这些单位。

```faml
[cache]
ttl = 5 minutes
max_size = 100 MB

[network]
timeout = 30 seconds
buffer_size = 4 KB
```

这类特殊数据类型的处理在 KCL 和 PKL 中也有相应的支持，但 FAML 将其作为语言内核的一部分，提供了更自然的语法支持。TOML 作为静态配置语言则需要通过字符串或数字配合注释的方式来表示这些特殊类型。

### 5. 强大的表达式系统

FAML 支持丰富的表达式操作，包括：
- 算术运算：`+`, `-`, `*`, `/`, `%`, `**`
- 比较运算：`==`, `!=`, `<`, `>`, `<=`, `>=`
- 逻辑运算：`&&`, `||`, `!`
- 三元条件运算符：`condition ? value1 : value2`
- 函数调用：`value.abs()`

```faml
[user]
age = 25
is_adult = age >= 18
welcome_message = is_adult ? $"Welcome, adult user!" : $"Welcome, young user!"
```

与 TOML 相比，FAML 的表达式系统显然更为丰富。KCL 和 PKL 同样具备强大的表达式系统，但在语法风格上各有特色。FAML 的表达式语法设计更贴近主流编程语言，降低了学习成本。

### 6. 灵活的数据结构

FAML 支持数组和映射结构，可以轻松定义复杂的数据结构。

```faml
[server]
ports = [8080, 8081, 8082]
middleware = {
  cors: { enabled: true, origins: ["*"] },
  compression: { enabled: true }
}
```

在数据结构支持方面，FAML 与 KCL、PKL 和 TOML 都提供了数组和映射（或称对象）的支持。FAML 的语法延续了 TOML 的风格。

### 7. 运行时动态修改

FAML 配置可以在运行时被程序动态修改，然后重新计算依赖这些值的其他配置项，实现真正的动态配置更新。

```rust
let mut config = FamlExpr::from_str(config_str)?;
config["server"]["port"].set_int(9000);  // 动态修改端口
let connection_string = config["database"]["connection_string"].evaluate()?.as_str();  // 自动更新连接字符串
```

这一特性是 FAML 相比于 TOML、KCL 和 PKL 的独特优势。KCL 和 PKL 主要用于构建时生成配置，而 FAML 支持运行时动态修改，更适合需要动态调整配置的应用场景。

FAML 通过这些特性，在保持配置语言简洁易用的同时，提供了强大的动态配置能力，解决了传统静态配置语言在动态场景下的局限性。
