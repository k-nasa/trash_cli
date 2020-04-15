// TODO: remove this line when the clap version used has this commit:
// https://github.com/clap-rs/clap/commit/56d182d98bb51401ae99d071a4fad772351dc1b4
use clap::{crate_authors, crate_description, crate_name, crate_version};

use clap::{app_from_crate, AppSettings, Arg, SubCommand};

fn main() {
    let matches = app_from_crate!()
        .setting(AppSettings::DeriveDisplayOrder)
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(SubCommand::with_name("clean").about("Clean trash directory"))
        .subcommand(
            SubCommand::with_name("rm").about("Trash file remove").arg(
                Arg::with_name("path")
                    .required(true)
                    .multiple(true)
                    .help("Remove file path"),
            ),
        )
        .get_matches();
    match matches.subcommand() {
        ("clean", Some(_)) => {
            trash::linux_windows::purge_all(trash::linux_windows::list().unwrap()).unwrap();
        }
        ("rm", Some(matches)) => trash::remove_all(matches.values_of("path").unwrap()).unwrap(),
        _ => {}
    }
}
