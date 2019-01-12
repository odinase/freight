#[macro_use]
extern crate clap;
extern crate freight;

use freight::new;
use std::env;

fn main() {
    println!(
        "Current working directory: {}",
        env::current_dir().unwrap().to_str().unwrap()
    );
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

    //println!("{:?}", args);

    let (_, sub_args) = args.subcommand();
    new::create_project(sub_args.unwrap());
}
