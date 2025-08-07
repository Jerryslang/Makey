use std::{env, fs, process::Command};
use toml::Value;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cwd = env::current_dir()?;
    let config_path = cwd.join("makey.conf");

    if !config_path.exists() {
        eprintln!("makey.conf not found in {}", cwd.display());
        std::process::exit(1);
    }

    let contents = fs::read_to_string(&config_path)?;
    let toml_value: Value = toml::from_str(&contents)?;

    let args: Vec<String> = env::args().collect();
    let section_name = if args.len() > 1 { &args[1] } else { "main" };

    let section = toml_value
        .get(section_name)
        .ok_or(format!("Section [{}] not found in makey.conf", section_name))?;

    let execute_template = section
        .get("execute")
        .and_then(Value::as_str)
        .ok_or(format!("No 'execute' string found in [{}]", section_name))?;

    let mut replacements = std::collections::HashMap::new();
    if let Value::Table(table) = section {
        for (k, v) in table.iter() {
            let s = match v {
                Value::String(s) => s.clone(),
                Value::Array(arr) => {
                    let mut parts = Vec::new();
                    for item in arr {
                        if let Value::String(str_item) = item {
                            parts.push(str_item.clone());
                        }
                    }
                    parts.join(" ")
                }
                Value::Integer(i) => i.to_string(),
                Value::Float(f) => f.to_string(),
                Value::Boolean(b) => b.to_string(),
                _ => "".to_string(),
            };
            replacements.insert(k.as_str(), s);
        }
    }

    let mut command_str = execute_template.to_string();
    for (key, val) in &replacements {
        let placeholder = format!("{{{}}}", key);
        command_str = command_str.replace(&placeholder, val);
    }

    println!("{}", command_str);

    let status = Command::new("sh")
        .arg("-c")
        .arg(&command_str)
        .status()?;

    if !status.success() {
        eprintln!("Command exited with status: {:?}", status.code());
        std::process::exit(status.code().unwrap_or(1));
    }

    Ok(())
}
