use clap::ArgMatches;
use os_info;
use std::process::Command;

pub fn create_project(args: &ArgMatches) {
    println!("project_name: {}", args.value_of("project_name").unwrap());
    Command::new("touch")
        .arg("hello.txt")
        .spawn()
        .expect("failed to execute process");
    //let os = get_os();
    let info = os_info::get();
    println!("Your OS is {}", info.os_type());
}

fn get_os() -> String {
    String::from_utf8_lossy(
        &Command::new("uname")
            .output()
            .expect("failed to execute uname")
            .stdout,
    ).to_string()
}
