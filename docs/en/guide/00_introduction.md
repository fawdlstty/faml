# Introduction

TOML is a concise configuration language with good readability and ease of use. It supports comments and nested structures, with intuitive syntax that makes it particularly suitable as a configuration file format. Compared to JSON and YAML, TOML is less prone to errors when manually edited.

As a static configuration language, TOML has limitations when dealing with dynamic configuration needs. For example, in Rust projects' Cargo.toml, conditional dependencies are typically represented as strings:

```toml
[dependencies]
[target.'cfg(not(target_env = "msvc"))'.dependencies]
tikv-jemallocator = { version = "0.6.0", features = ["profiling", "unprefixed_malloc_on_supported_platforms"] }
```

In this case, `cfg(not(target_env = "msvc"))` is treated as a string, parsed as a conditional expression by the build tool. This approach can lead to the following problems:
1. Mixing different language syntaxes increases configuration complexity
2. Embedded expressions lack explicit syntax specifications

For dynamic configuration needs, there are already some solutions in the industry:

**KCL**

A dynamic configuration language hosted by CNCF, example:

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

A dynamic configuration language developed by Apple, example:

```pkl
name = "Swallow"

job {
  title = "Sr. Nest Maker"
  company = "Nests R Us"
  yearsOfExperience = 2
}
```

Existing dynamic configuration languages typically use JSON-like structured organization, which differs from TOML's style. Additionally, each language has its specific syntax rules, requiring additional learning costs.

## Core Advantages of FAML

FAML aims to combine TOML's concise syntax with dynamic configuration capabilities, providing a configuration language choice that maintains TOML's usability while having dynamic features. Compared to other dynamic configuration languages, FAML has the following unique advantages:

### 1. Clean Syntax, Easy to Learn

FAML continues TOML's clean syntax style, using square brackets to define configuration blocks and equals signs for assignment, maintaining the same readability as TOML. For developers familiar with TOML, learning FAML has almost no learning curve.

```faml
[server]
port = 8080
host = "localhost"
```

In contrast, KCL uses curly braces `{}` to define objects, closer to JSON syntax; PKL adopts a struct-like definition method. FAML's TOML-style syntax gives it advantages in readability and writability, especially when manually editing configuration files.

### 2. Powerful Dynamic Expression Support

FAML supports embedding expressions in configurations, allowing references to other configuration items, arithmetic operations, logical judgments, and other operations to achieve truly dynamic configuration.

```faml
[database]
host = "localhost"
port = 5432
connection_string = $"postgresql://{host}:{port}/mydb"
```

Compared to TOML, FAML's expression functionality is more powerful. As a static configuration language, TOML does not support expression calculation, while FAML's `$""` syntax allows embedding expressions in strings to generate dynamic content. KCL and PKL also support similar expression functionality, but FAML's syntax is more concise and clear.

### 3. Conditional Configuration Control

Through the `@if` directive, FAML can control whether configuration items take effect based on conditions, which is very useful in configuration management for different environments.

```faml
[app]
env = "production"

@if env == "development"
log_level = "debug"

@if env == "production"
log_level = "error"
```

This feature makes FAML superior to standard TOML in conditional configuration because TOML itself does not support conditional logic. Although KCL and PKL can also implement conditional configuration, FAML's `@if` syntax is more intuitive and conforms to configuration file usage habits.

### 4. Built-in Special Data Types

FAML has built-in support for special data types such as duration and distance, which can be used directly in configurations.

```faml
[cache]
ttl = 5 minutes
max_size = 100 MB

[network]
timeout = 30 seconds
buffer_size = 4 KB
```

This type of special data type handling is also supported in KCL and PKL, but FAML treats it as part of the language kernel, providing more natural syntax support. As a static configuration language, TOML needs to use strings or numbers with comments to represent these special types.

### 5. Powerful Expression System

FAML supports rich expression operations, including:
- Arithmetic operations: `+`, `-`, `*`, `/`, `%`, `**`
- Comparison operations: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical operations: `&&`, `||`, `!`
- Ternary conditional operator: `condition ? value1 : value2`
- Function calls: `value.abs()`

```faml
[user]
age = 25
is_adult = age >= 18
welcome_message = is_adult ? $"Welcome, adult user!" : $"Welcome, young user!"
```

Compared to TOML, FAML's expression system is obviously richer. KCL and PKL also have powerful expression systems, but each has its own syntax style. FAML's expression syntax design is closer to mainstream programming languages, reducing learning costs.

### 6. Flexible Data Structures

FAML supports arrays and mapping structures, making it easy to define complex data structures.

```faml
[server]
ports = [8080, 8081, 8082]
middleware = {
  cors: { enabled: true, origins: ["*"] },
  compression: { enabled: true }
}
```

In terms of data structure support, FAML, along with KCL, PKL, and TOML, provides support for arrays and mappings (or objects). FAML's syntax continues TOML's style.

### 7. Runtime Dynamic Modification

FAML configurations can be dynamically modified by programs at runtime, then recalculate other configuration items that depend on these values, achieving true dynamic configuration updates.

```rust
let mut config = FamlExpr::from_str(config_str)?;
config["server"]["port"].set_int(9000);  // Dynamically modify port
let connection_string = config["database"]["connection_string"].evaluate()?.as_str();  // Automatically update connection string
```

This feature is a unique advantage of FAML compared to TOML, KCL, and PKL. KCL and PKL are mainly used for build-time configuration generation, while FAML supports runtime dynamic modification, making it more suitable for application scenarios that require dynamic configuration adjustments.

FAML addresses the limitations of traditional static configuration languages in dynamic scenarios while maintaining the simplicity and ease of use of configuration languages through these features.