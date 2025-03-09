use std::fs::read_to_string;

const FILE_PATH: &str = "/etc/hosts";
const LOCALHOST: &str = "127.0.0.1";
fn main() -> Result<(), std::io::Error> {
    println!("Hello, Let's focus!");
    let mut file: String = read_to_string(FILE_PATH).expect("Failed to read file");

    let web_site: &str = "www.example.com\n";
    file.push_str(LOCALHOST);
    file.push_str("               ");
    file.push_str(web_site);
    println!("{}", file);

    let res = std::fs::write(FILE_PATH, file);
    return res;
}
