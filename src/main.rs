#[path = "./utils/utils.rs"]
mod utils;

#[path = "./hosts/file_edit.rs"]
mod file_edit;

fn main() -> Result<(), std::io::Error> {
    println!("Hello, Let's focus!");

    let arguments: Vec<String> = std::env::args().map(|arg| arg).collect();
    let match_arg = utils::match_args(&arguments);
    println!("{:?}", match_arg);
    let res = file_edit::parse_hosts_file(&match_arg);

    return res;
}
