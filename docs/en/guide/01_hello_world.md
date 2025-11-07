# Getting Started

Suppose we now have a faml configuration language with the following content:

```faml
[hello]
value = 12
name = $"hello {value + 12}"
```

We want to use Rust to modify the hello.value property and then get the hello.name property value. Let's create a Rust project and add the faml dependency:

```bash
cargo new hello_faml
cd hello_faml
cargo add faml
cargo add anyhow
```

Note: Since the example code uses the `anyhow` library for error handling, we need to add the `anyhow` dependency as well.

The Cargo.toml file should contain the following dependencies:

```toml
[dependencies]
faml = "0.1"
anyhow = "1.0"
```

Add the example code:

```rust
use anyhow::Result;

fn main() -> Result<()> {
    let faml_str = r#"
[hello]
value = 12
name = $"hello {value + 12}"
"#;
    let mut root = faml::FamlExpr::from_str(faml_str)?;
    root["hello"]["value"].set_int(30);
    let name = root["hello"]["name"].evaluate()?.as_str();
    println!("{}", name); // hello 42
    Ok(())
}
```

After execution, it will print the hello.name property value: `hello 42`

## Code Explanation

1. First, we import the `anyhow::Result` type for error handling
2. Use `faml::FamlExpr::from_str()` to parse the faml string
3. Modify the value to 30 through `root["hello"]["value"].set_int(30)`
4. Calculate and get the name value through `root["hello"]["name"].evaluate()?.as_str()`
5. Since value is modified to 30, the result of the expression `value + 12` is 42, so the final output is `hello 42`