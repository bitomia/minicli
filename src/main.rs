fn main() {
    let cli = minicli::CliArgs::parse();

    match cli.command.as_deref() {
        Some("clients") => handle_clients(&cli),
        Some("help") | None => print_help(&cli),
        Some(other) => {
            eprintln!("Unknown subcommand: {}", other);
            print_help(&cli);
        }
    }
}

fn handle_clients(cli: &minicli::CliArgs) {
    if cli.has_flag("help") || cli.subcommands.len() == 0 {
        print_help(cli);
        return;
    }

    let subcommand = cli.subcommands[0].clone();
    match subcommand.as_str() {
        "ls" => {
            println!("Listing all clients");
        }
        _ => {
            print_help(&cli);
        }
    }
}

fn print_help(cli: &minicli::CliArgs) {
    let program = cli.program.clone().unwrap();
    println!(
        r#"Usage:
    {} <command> [options]

Commands:
    clients       Manage clients
    help          Show this help message

Example:
    {} clients ls
"#,
        program, program,
    );
}
