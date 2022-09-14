use std::fs;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Dir {
    path: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct File {
    path: String,
    state: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Template {
    src: String,
    dest: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
enum Command {
    File(File),
    Template(Template),
    Dir(Dir),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Provision {
    provision: Vec<Command>,
}

fn ymling() {
    let do_steps = || -> Result<(), Box<dyn std::error::Error>> {
        let file = fs::read_to_string("./src/provision.yml")?;
        let provision: Provision = serde_yaml::from_str(&file)?;
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
