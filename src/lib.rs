#[macro_use]
extern crate clap;

use clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

pub fn build_app() -> App<'static, 'static> {
    App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .setting(AppSettings::DeriveDisplayOrder)
        .subcommand(SubCommand::with_name("clean").about("Clean trash directory"))
        .subcommand(SubCommand::with_name("config").about("Open config file"))
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
