use std::fs;

use serde;
use serde::Deserialize;
use serde::Serialize;
use serde_yaml;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Variables {}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct VariablesCommand {
    name: Option<String>,
    include_vars: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct File {
    path: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct DirCommand {
    name: Option<String>,
    dir: File,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct FileCommand {
    name: Option<String>,
    file: File,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TemplateFile {
    src: Option<String>,
    dest: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct TemplateCommand {
    name: Option<String>,
    template: TemplateFile,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Command {
    FileCommand(FileCommand),
    TemplateCommand(TemplateCommand),
    DirCommand(DirCommand),
    VariablesCommand(VariablesCommand),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
struct Provision {
    provision: Vec<Command>,
}

fn ymling() {
    let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
        let path = Path::new("./provision1.yml");
        let file = fs::read_to_string(path)?;
        println!("file: {}", file);
        let provision: Provision = serde_yaml::from_str(&file).unwrap();
        println!("{:#?}", provision);
        Ok(())
    };
    do_steps().unwrap();
}

fn cli() -> clap::Command<'static> {
    clap::Command::new("mooncake")
        .about("Mooncake provisioning tool")
        .version("0.1.0")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(clap::Command::new("roles").about("Prints the found roles"))
        .subcommand(clap::Command::new("run").about("Runs the provisioning"))
}

fn roles() {
    println!("roles");
}

fn run() {
    let mut context = tera::Context::new();
    let mut tera = tera::Tera::default();

    context.insert("name", &"Mooncake");
    let result = tera.render_str("Hello {{name}}!", &context);
    match result {
        Ok(string) => println!("{}", string),
        Err(error) => panic!("Problem rendering {:?}", error),
    }
}

fn main() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("roles", _)) => roles(),
        Some(("run", _)) => run(),
        Some(("ymling", _)) => ymling(),
        _ => println!("No subcommand was used"),
    }
}
