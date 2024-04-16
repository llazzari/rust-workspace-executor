
use std::process::{Command, ExitStatus};
use clap::{App, Arg};


fn main() {
    // Define command-line interface using clap
    let matches = App::new("Workspace Executor")
        .version("1.0")
        .author("Lucas Lazzari <lucas.lazzari@outlook.com>")
        .about("Executes different workspaces")
        .arg(
            Arg::with_name("workspace")
                .help("Specifies the workspace to run")
                .possible_values(&["thesis", "physics-project", "psychology-app"])
                .required(true),
        )
        .get_matches();

    // Retrieve and match the selected workspace
    let workspace = matches.value_of("workspace").unwrap();
    match workspace {
        "thesis" => thesis(),
        "physics-project" => physics_project(),
        "psychology-app" => psychology_app(),
        _ => eprintln!("Invalid workspace specified"),
    }
}

fn thesis() {
    let commands = [
        ("google-chrome", None),
        ("xdg-open", Some("/home/llazzari/Física/tese/tese.pdf")),
        ("xdg-open", Some("/home/llazzari/Física/tese/jonas.pdf")),
        ("code", Some("/home/llazzari/Física/tese")),
    ];
    execute_commands(&commands);
}

fn physics_project() {
    let commands = [
        ("google-chrome", None),
        ("xdg-open", Some("/home/llazzari/Física/projeto/projeto.pdf")),
        ("code", Some("/home/llazzari/Física/projeto")),
    ];
    execute_commands(&commands);
}

fn psychology_app() {
    let commands = [
        ("google-chrome", None),
        ("code", Some("/home/llazzari/Projects/psychology-apps/prontuary")),
    ];
    execute_commands(&commands);
}

fn execute_commands(commands: &[(&str, Option<&str>)]) {
    for (command, arg) in commands {
        match execute_command(command, *arg) {
            Ok(_) => println!("Successfully executed: {} {:?}", command, arg),
            Err(err) => eprintln!("Error executing {}: {:?}", command, err),
        }
    }
}

fn execute_command(command: &str, arg: Option<&str>) -> Result<ExitStatus, std::io::Error> {
    let mut cmd = Command::new(command);
    if let Some(arg) = arg {
        cmd.arg(arg);
    }
    cmd.spawn()?.wait()
}
