use clap::ArgMatches;

pub fn create_project(args: &ArgMatches) {
    const DEFAULT_PROJECT_NAME: &'static str = "cpp-project";
    let project_name = match args.value_of("project_name") {
        Some(name) => name,
        None => DEFAULT_PROJECT_NAME,
    };
}
