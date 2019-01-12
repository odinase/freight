use clap::ArgMatches;
use os_info;
use std::env;

pub struct AppHandle<'a> {
    args: ArgMatches<'a>,
    working_dir: String,
    os: String,
}

impl<'a> AppHandle<'a> {
    pub fn new() -> Self {
        let args = clap_app!(Freight =>
        (version: crate_version!())
        (author: "Odin Aleksander Severinsen <odin.sev@gmail.com>")
        (about: "Cargo clone for C/C++")
        (@arg fltk: -f --fltk "Sets up the project as FLTK")
        (@subcommand new =>
            (@arg project_name: index(1) "Custom name of project")
            (@arg FLTK: --fltk "Sets up the project as FLTK")
        )
        )
        .get_matches();
        let working_dir = String::from(env::current_dir().unwrap().to_str().unwrap());
        let os = os_info::get().to_string();

        AppHandle {
            args,
            working_dir,
            os,
        }
    }
}
