use std::{io::BufReader, process::Command, thread};

use serde_json::Value;
struct ExcCommand {
    name: String,
    content: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut commands: Vec<ExcCommand> = vec![];
    let config_path = "config.json";
    let config = read_config(config_path)?;
    if let Value::Object(map) = config {
        for (key, value) in map {
            let command = ExcCommand {
                name: key.clone(),
                content: value.as_str().unwrap_or("").to_string(),
            };
            commands.push(command);
        }
    }
    for command in commands {
        thread::spawn(move || {
            println!("{}: {}", command.name, command.content);
            Command::new("osascript")
                .arg("-e")
                .arg(format!(
                    "tell application \"Terminal\" to do script \"{}\"",
                    command.content
                ))
                .output()
        })
        .join()
        .unwrap()?;
    }
    loop {}
}
pub fn read_config<P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Value, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}
