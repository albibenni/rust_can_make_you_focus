use std::u64;

mod utils;

mod hosts;

const RESET_FILE_PATH: &str = "
##
# Host Database
#
# localhost is used to configure the loopback interface
# when the system is booting.  Do not change this entry.
##
127.0.0.1		localhost
255.255.255.255		broadcasthost
::1                          localhost
";
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
        utils::utils::help();
        return Ok(());
    }

    let last_arg: String = arguments.pop().expect("Something went wrong");
    let sleep_time: u64 = utils::utils::parse_sleep_time(&last_arg);
    let match_arg: Vec<&str> = utils::utils::match_args(&arguments);

    let parse_host_files = hosts::file_edit::parse_hosts_file(&match_arg);

    if parse_host_files.is_err() {
        return parse_host_files;
    }
    let res_flux = utils::utils::execute_flux_cache();
    if res_flux.is_err() {
        return res_flux;
    }
    utils::utils::plan_sleep(sleep_time);
    return utils::utils::reset_file(RESET_FILE_PATH);
}
