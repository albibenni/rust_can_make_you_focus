#[path = "./utils/utils.rs"]
mod utils;

const FILE_PATH: &str = "/etc/hosts";
const LOCALHOST: &str = "127.0.0.1";
fn main() -> Result<(), std::io::Error> {
    println!("Hello, Let's focus!");
    let mut file: String = std::fs::read_to_string(FILE_PATH).expect("Failed to read file");

    let arguments: Vec<String> = std::env::args().map(|arg| arg).collect();
    let res = utils::match_args(&arguments);
    println!("{:?}", res);

    // let web_site: &str = "www.example.com\n";
    // file.push_str(LOCALHOST);
    // file.push_str("               ");
    // file.push_str(web_site);
    // println!("{}", file);
    //
    // let res = std::fs::write(FILE_PATH, file);
    // return res;
    return Ok(());
}
