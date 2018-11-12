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
use std::process::Command;
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
        .subcommand(
            SubCommand::with_name("rm")
                .about("Trash file remove")
                .arg(Arg::with_name("path").help("Remove file path")),
        )
}

pub fn run() {
    let mut app = build_app();

    match app.clone().get_matches().subcommand() {
        ("clean", Some(_)) => println!("clean!"),
        ("dir", Some(matches)) => cmd_config(&matches),
        ("rm", Some(_)) => println!("rm!"),
        _ => {
            app.print_long_help().ok();
            return;
        }
    }
}

fn cmd_config(matches: &ArgMatches) {
    let mut config = Config::load_config().unwrap();

    let dir = match dirs::home_dir() {
        Some(dir) => Path::new(&dir.to_str().unwrap().to_string()).join(".config/trash_cli/"),
        None => panic!("faild fetch home_dir name"),
    };
    let filepath = &dir.join("config.toml");

    let mut editor_process = Command::new("vim")
        .arg(filepath)
        .spawn()
        .expect("Failed open editor");

    editor_process.wait().expect("Failed to run");
}

    let input_filepath = match matches.value_of("path") {
        Some(path) => path.to_string(),
        None => {
            print!("Input new trash_dir path: ");
            std::io::stdout().flush().expect("print! is faild");
            read()
        }
    };

    config
        .set_new_trash_dir_path(input_filepath)
        .expect("To set new trash dir is faild");
}

fn cmd_rm(matches: &ArgMatches) {}
fn cmd_clean(matches: &ArgMatches) {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    trash_dir_path: String,
}

impl Config {
    pub fn set_new_trash_dir_path(&mut self, dir_path: String) -> Result<()> {
        self.trash_dir_path = dir_path;
        let toml_str = toml::to_string(self).unwrap();
        let mut file = Config::load_or_create_config_file();

        file.write_all(toml_str.as_bytes())
    }

    pub fn load_config() -> Result<Config> {
        let mut file = Config::load_or_create_config_file();

        let mut buf = vec![];
        file.read_to_end(&mut buf)?;
        let toml_str = match from_utf8(&buf) {
            Ok(toml_str) => toml_str,
            Err(e) => panic!(e),
        };

        let config: Config = if toml_str.is_empty() {
            let config = Config::default();
            let toml_str = toml::to_string(&config).unwrap();

            match file.write_all(toml_str.as_bytes()) {
                Ok(_) => config,
                Err(e) => panic!(e),
            }
        } else {
            match toml::from_str(toml_str) {
                Ok(config) => config,
                _ => Config::default(),
            }
        };

        Ok(config)
    }

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

fn home_dir_string() -> String {
    match dirs::home_dir() {
        Some(dir) => dir.to_str().unwrap().to_string(),
        _ => panic!("Home directory is not set"),
    }
}

fn read<T: FromStr>() -> T {
    let mut s = String::new();
    stdin().read_line(&mut s).ok();
    s.trim().parse().ok().unwrap()
}

impl Default for Config {
    fn default() -> Self {
        let trash_dir_path = home_dir_string() + "/.Trash";

        Config { trash_dir_path }
    }
}
