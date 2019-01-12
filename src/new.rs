use clap::ArgMatches;
use std::process::Command;

pub fn create_project(args: &ArgMatches) {
    println!("project_name: {}", args.value_of("project_name").unwrap());
    Command::new("touch")
        .arg("hello.txt")
        .spawn()
        .expect("failed to execute process");
    let os = get_os();
    println!("Your OS is {}", os);
    Command::new("echo")
        .arg("$(pwd)")
        .spawn()
        .expect("failed to execute process");
}

fn get_os() -> String {
    String::from_utf8_lossy(
        &Command::new("uname")
            .output()
            .expect("failed to execute uname")
            .stdout,
    )
    .to_string()
}

enum Os {
    Windows,
    Linux,
    Mac,
}
//
//impl Os {
//    fn new() -> Self {}
//}
//
