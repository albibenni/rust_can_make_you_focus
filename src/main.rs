#[path = "./utils/utils.rs"]
mod utils;

#[path = "./hosts/file_edit.rs"]
mod file_edit;

const FILE_PATH: &str = "src/utils/reset_default_file.txt";

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Let's focus!");

    let arguments: Vec<String> = std::env::args().map(|arg| arg).collect();
    let match_arg = utils::match_args(&arguments);
    println!("{:?}", match_arg);
    let parse_host_files = file_edit::parse_hosts_file(&match_arg);

    if parse_host_files.is_err() {
        return parse_host_files;
    }
    let reset_res = utils::reset_file(FILE_PATH);
    //let res = std::fs::write(FILE_PATH, reset_file);
    println!("{:?}", reset_res);
    return reset_res;
}
