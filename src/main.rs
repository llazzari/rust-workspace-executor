use std::process::{Command, ExitStatus};
use structopt::StructOpt;

// Define a struct to represent command-line arguments
#[derive(Debug, StructOpt)]
enum Workspace {
    #[structopt(name = "thesis")]
    Thesis,
    #[structopt(name = "physics-project")]
    PhysicsProject,
    #[structopt(name = "psychology-app")]
    PsychologyApp,
}

fn main() {
    // Parse command-line arguments
    let workspace = Workspace::from_args();

    // Run the selected workspace
    match workspace {
        Workspace::Thesis => thesis(),
        Workspace::PhysicsProject => physics_project(),
        Workspace::PsychologyApp => psychology_app(),
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
