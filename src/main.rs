#[macro_use]
extern crate clap;
extern crate freight;

use freight::new;
use std::process::Command;

fn main() {
    let args = clap_app!(Freight =>
    (version: crate_version!())
    (author: "Odin Aleksander Severinsen <odin.sev@gmail.com>")
    (about: "Cargo clone for C/C++")
    (@arg fltk: -f --fltk "Sets up the project as FLTK")
    (@subcommand new =>
        (@arg project_name: index(1) "Custom name of project")
        (@arg FLTK: --fltk "Sets up the project as FLTK")
    )
    ).get_matches();

    match args.subcommand() {
        ("new", Some(sub_args)) => new::create_project(sub_args),
        _ => (),
    }
}
//
//let args = clap_app!(Duma =>
//        (version: crate_version!())
//        (author: "Matt Gathu <mattgathu@gmail.com>")
//        (about: "wget clone written in Rust")
//        (@arg quiet: -q --quiet "quiet (no output)")
//        (@arg continue: -c --continue "resume getting a partially-downloaded file")
//        (@arg FILE: -O --output +takes_value "write documents to FILE")
//        (@arg AGENT: -U --useragent +takes_value "identify as AGENT instead of Duma/VERSION")
//        (@arg SECONDS: -T --timeout +takes_value "set all timeout values to SECONDS")
//        (@arg URL: +required +takes_value "url to download")
//        ).get_matches();
