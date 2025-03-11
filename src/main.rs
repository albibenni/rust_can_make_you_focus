use utils::execute_flux_cache;

#[path = "./utils/utils.rs"]
mod utils;

#[path = "./hosts/file_edit.rs"]
mod file_edit;

const RESET_FILE_PATH: &str = "src/utils/reset_default_file.txt";
const FILE_PATH: &str = "/etc/hosts";
const LOCALHOST: &str = "127.0.0.1";

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Let's focus!");

    let mut arguments: Vec<String> = std::env::args().map(|arg| arg).collect();
    if arguments.len() <= 1 {
        panic!("arguments must be defined");
    }
    if arguments
        .get(1)
        .expect("No arguments provided")
        .to_lowercase()
        == "help"
    {
        utils::help();
        return Ok(());
    }

    let sleep_time: u64 = arguments
        .pop()
        .expect("Something went wrong")
        .parse::<u64>()
        .expect("You didn't provide a number");
    let match_arg = utils::match_args(&arguments);
    let parse_host_files = file_edit::parse_hosts_file(&match_arg);

    if parse_host_files.is_err() {
        return parse_host_files;
    }
    let res_flux = execute_flux_cache();
    if res_flux.is_err() {
        return res_flux;
    }
    // todo add sleep time based on arg timer for pomodoro
    utils::plan_sleep(sleep_time);
    return utils::reset_file(RESET_FILE_PATH);
}
