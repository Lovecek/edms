#[allow(unused_imports)] //delete later
use std::path::PathBuf;
use std::fs;
use clap::{arg, command, value_parser, ArgAction, Command};

#[derive(Debug)]
struct FileEntry {
    path: PathBuf,
    name: String,
}

fn main() {
    let mut file_list: Vec<FileEntry> = Vec::new();

    let matches = command!()
        .arg(arg!(<edms>))
        .subcommand(
            Command::new("add")
            .about("Add a file to the list of files to be encrypted")
            .arg(arg!(<file> "The file to add")
                .value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("remove")
            .about("Remove a file from the list of files to be encrypted")
            .arg(arg!(<file> "The file to remove")
                .value_parser(clap::value_parser!(PathBuf)))
        )
        .subcommand(
            Command::new("list")
            .about("Shows files in list to be encrypted")
        )
        .subcommand(
            Command::new("encrypt")
            .about("Encrypts files currently in the list")
        )
        .get_matches();
    
    match matches.subcommand() {
        Some(("add", sub_m)) => {
            let file: &PathBuf = sub_m.get_one::<PathBuf>("file").unwrap();

            let abs_path = fs::canonicalize(file)
                .expect("Could not resolve absolute path");
            println!("Absolute path : {:?}", abs_path);

            let name = abs_path.file_name()
                .unwrap()
                .to_string_lossy()
                .to_string();

            file_list.push(FileEntry {
                path: abs_path,
                name,
            });

            println!("File added to list")
        }
        Some(("remove", sub_m)) => {
            let file: &PathBuf = sub_m.get_one::<PathBuf>("file").unwrap();
            println!("Removing {:?} from list", file);
        }
        Some(("list", _)) => {
            if file_list.is_empty() {
                println!("No files in list!")
            } else {
                println!("Files in list:");
                for f in &file_list {
                    println!("- {} ({:?})", f.name, f.path)
                }
            };
        }
        Some(("encrypt", _)) => {
            println!("Files succesfully encrypted!")
        }
        _ => {
            println!("Use --help to see avaliable commands");
        }
    }

    //Create a list for to-be encrypted files

}
