extern crate cgats;
use cgats::*;

#[macro_use]
extern crate clap;
use clap::{ArgMatches, Arg, App, SubCommand};

// use std::fs::File;
// use std::path::Path;
// use std::io::BufReader;
// use std::io::BufRead;
// use std::collections::HashMap;
// use std::io::stdin;

#[derive(Debug)]
enum Command {
    Average,
    // Convert,
}

impl Command {
    fn from_string(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "average" | "avg" => Some(Command::Average),
            // "convert"         => Some(Command::Convert),
            _ => None
        }
    }

    fn execute(&self, cgv: CgatsVec) -> CgatsResult<CgatsObject> {
        match &self {
            Command::Average => cgv.average()
        }
    }
}

#[derive(Debug)]
struct Config {
    command: Option<Command>,
    files: Vec<String>,
}

impl<'a> Config {
    fn build(matches: &ArgMatches) -> Self {
        let cmd_name = matches.subcommand_name();
        let command = match cmd_name {
            Some(cmd) => Command::from_string(cmd),
            None => None
        };

        let files = if let Some(cmd) = cmd_name {
            match matches.subcommand_matches(cmd) {
                Some(scm) => scm.values_of("comparefiles").unwrap().map(|m| m.to_string()).collect(),
                None => Vec::new()
            }
        } else {
            let clap_files = matches.values_of("files");
            match clap_files {
                Some(f) => f.map(|m| m.to_string()).collect(),
                None => Vec::new()
            }
        };

        Self { command, files }
    }

    fn execute(&self) -> CgatsResult<CgatsObject> {
        match &self.command {
            Some(cmd) => cmd.execute(self.cgats_vec()?),
            None => Err(CgatsError::CannotCompare)
        }
    }

    fn cgats_vec(&self) -> CgatsResult<CgatsVec> {
        CgatsVec::from_files(&self.files)
    }
}

fn main() -> CgatsResult<()> {
    //Parse command line arguments with clap
    let matches = App::new("cgats")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::with_name("files")
            .help("CGATS files")
            .value_name("FILE")
            .multiple(true))
        .subcommand(SubCommand::with_name("average")
            .about("Average 2 or more CGATS color files")
            .arg(Arg::with_name("comparefiles")
                .value_name("FILE")
                .multiple(true)
                .required(true))
            )
        .get_matches();

    let config = Config::build(&matches);

    if config.files.len() < 1 {
        eprintln!("{}", matches.usage());
        std::process::exit(1);
    }

    match &config.command {
        Some(_) => {
            let cgo = config.execute()?;
            println!("{}", cgo.print()?);
        },
        None => {
            for file in &config.files {
                match CgatsObject::from_file(&file) {
                    Ok(cgo) => println!("'{}':\n\t{}", file, cgo),
                    Err(e) => eprintln!("'{}':\n\t{}", file, e)
                }
            }
        }
    }

    Ok(())
}