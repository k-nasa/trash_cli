#[macro_use]
extern crate clap;
#[macro_use]
extern crate serde_derive;
extern crate dirs;
extern crate serde;
extern crate toml;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};
use std::fs::*;
use std::io::*;
use std::path::*;
use std::str::from_utf8;
use std::str::FromStr;

pub fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(SubCommand::with_name("clean").about("Clean trash directory"))
        .subcommand(
            SubCommand::with_name("dir")
                .about("Config trash directory")
                .arg(Arg::with_name("path").help("New trash dir absolute path")),
        )
        .subcommand(SubCommand::with_name("rm").about("Trash file remove"))
}

pub fn run() {
    let mut app = build_app();

    match app.clone().get_matches().subcommand() {
        ("clean", Some(_)) => println!("clean!"),
        ("config", Some(_)) => println!("config!"),
        ("rm", Some(_)) => println!("rm!"),
        _ => {
            app.print_long_help().ok();
            return;
        }
    }
}

fn cmd_config(matches: &ArgMatches) {}
fn cmd_rm(matches: &ArgMatches) {}
fn cmd_clean(matches: &ArgMatches) {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    trash_dir_path: String,
}

impl Config {
    fn load_or_create_config_file() -> File {
        let dir = match dirs::home_dir() {
            Some(dir) => Path::new(&dir.to_str().unwrap().to_string()).join(".config/trash_cli/"),
            None => panic!("faild fetch home_dir name"),
        };

        DirBuilder::new()
            .recursive(true)
            .create(dir.clone())
            .unwrap();

        let filepath = &dir.join("config.toml");

        match OpenOptions::new()
            .create(true)
            .write(true)
            .append(true)
            .read(true)
            .open(filepath)
        {
            Ok(file) => file,
            Err(e) => panic!(e),
        }
    }
}
