#[allow(unused_imports)] //delete later
use std::path::PathBuf;
use clap::{arg, command, value_parser, ArgAction, Command};

fn main() {
    let matches = command!()
        .arg(arg!(<edms>))
        .subcommand(
            Command::new("add")
            .about("Add a file to the list of files to be encrypted")
            .arg(arg!(<file> "The file to add"))
        )
        .subcommand(
            Command::new("remove")
            .about("Remove a file from the list of files to be encrypted")
            .arg(arg!(<file> "The file to remove"))
        )
        .subcommand(
            Command::new("list")
            .about("Shows files in list to be encrypted")
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
            println!("Adding {file} to list")
        }
        Some(("remove", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
            println!("Removing {file} from list");
        }
        Some(("list", _)) => {
            println!("Listing all files")
        }
        _ => {}
    }
}
