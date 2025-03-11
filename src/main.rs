#[path = "./utils/utils.rs"]
mod utils;

#[path = "./hosts/file_edit.rs"]
mod file_edit;

const RESET_FILE_PATH: &str = "src/utils/reset_default_file.txt";
const FILE_PATH: &str = "/etc/hosts";
const LOCALHOST: &str = "127.0.0.1";

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Let's focus!");

    let arguments: Vec<String> = std::env::args().map(|arg| arg).collect();
    let match_arg = utils::match_args(&arguments);
    let parse_host_files = file_edit::parse_hosts_file(&match_arg);

    if parse_host_files.is_err() {
        return parse_host_files;
    }
    // todo add sleep time based on arg timer for pomodoro
    //sleep(std::time::Duration::new(1, 0));
    return utils::reset_file(RESET_FILE_PATH);
}
