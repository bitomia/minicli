use std::collections::HashMap;
use std::env;

#[derive(Debug)]
pub struct CliArgs {
    pub program: Option<String>,
    pub command: Option<String>,
    pub subcommands: Vec<String>,
    pub options: HashMap<String, Option<String>>,
}

impl CliArgs {
    pub fn parse() -> Self {
        let mut args = env::args();
        let program = args.next();

        let mut command = None;
        let mut subcommands = Vec::new();
        let mut options = HashMap::new();

        while let Some(arg) = args.next() {
            if command.is_none() && !arg.starts_with("--") {
                command = Some(arg);
                break;
            }
        }

        for arg in args {
            if arg.starts_with("--") {
                let trimmed = &arg[2..];
                if let Some(eq_pos) = trimmed.find('=') {
                    let key = trimmed[..eq_pos].to_string();
                    let value = trimmed[eq_pos + 1..].to_string();
                    options.insert(key, Some(value));
                } else {
                    options.insert(trimmed.to_string(), None);
                }
            } else {
                subcommands.push(arg);
            }
        }

        CliArgs {
            program,
            command,
            subcommands,
            options,
        }
    }

    pub fn get_flag(&self, key: &str) -> Option<&str> {
        self.options.get(key).and_then(|v| v.as_deref())
    }

    pub fn has_flag(&self, key: &str) -> bool {
        self.options.contains_key(key)
    }
}
