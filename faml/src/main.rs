use std::io::Write;

fn read_line(tip: &str) -> anyhow::Result<String> {
    print!("{tip}");
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    input = input.trim().to_string();
    Ok(input)
}

fn main() -> anyhow::Result<()> {
    println!(
        "{} {}",
        std::env!("CARGO_PKG_NAME"),
        std::env!("CARGO_PKG_VERSION")
    );
    let mut expr = faml::FamlExpr::new();
    'main: loop {
        let mut input = read_line("> ")?;
        if input == "exit" || input == "quit" {
            break;
        } else if input.is_empty() {
            continue;
        } else if input.starts_with('[') {
            loop {
                let input2 = read_line(">>> ")?;
                if input2 == "exit" || input2 == "quit" {
                    break 'main;
                } else if input2.is_empty() {
                    break;
                } else if input2.starts_with('[') {
                    input.push('\n');
                    let expr2 = faml::FamlExpr::from_str(&input)?;
                    match expr.is_none() {
                        true => expr = expr2,
                        false => expr.apply(expr2)?,
                    }
                    input = input2;
                } else {
                    input.push('\n');
                    input.push_str(&input2);
                }
            }
            input.push('\n');
            let expr2 = faml::FamlExpr::from_str(&input)?;
            match expr.is_none() {
                true => expr = expr2,
                false => expr.apply(expr2)?,
            }
        } else {
            let tmp_expr = {
                let mut tmp_expr = faml::FamlExpr::expr_from_str(&input)?;
                let weak = expr.to_weak();
                tmp_expr.init_weak_expr(weak.clone(), weak);
                tmp_expr
            };
            let val = tmp_expr.evaluate()?;
            println!("{}", val.as_print_str());
        }
    }
    Ok(())
}
