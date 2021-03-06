use std::process;

use base64;
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use md5;
use regex::Regex;
use rpassword::read_password_from_tty;

fn print_usage() {
    let usage = "Generate a unique password for each account using a unique nickname and a master password

Usage:
    oplop [new | -h]

Options:
    new        Generate password for new account (requires entering nickname and master password twice)
     -h        Show this help message
";
    println!("{}", usage);
}

fn oplop_hash(label: &str, master: &str) -> String {
    let digest = md5::compute(format!("{}{}", master, label));
    base64::encode_config(&*digest, base64::URL_SAFE)
}

fn oplop_password(hash: &str) -> String {
    let re = Regex::new(r"\d+").unwrap();
    match re.find(hash) {
        Some(m) => {
            if m.start() < 8 {
                hash[0..8].to_owned()
            } else {
                let prefix_len = m.end() - m.start();
                m.as_str().to_owned() + &hash[..8 - prefix_len]
            }
        }
        None => "1".to_owned() + &hash[..7],
    }
}

fn set_clipboard(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.to_owned())
        .expect("Error copying to clipboard.");
    println!("Account password copied to clipboard!");
}

fn oplop() {
    let nickname = read_password_from_tty(Some("Enter account nickname: ")).unwrap();
    let password = read_password_from_tty(Some("Enter master password:")).unwrap();
    let hash = oplop_hash(&nickname, &password);
    let password = oplop_password(&hash);
    set_clipboard(&password);
}

fn oplop_new() {
    let nickname = read_password_from_tty(Some("Enter account nickname: ")).unwrap();
    let password = read_password_from_tty(Some("Enter master password:")).unwrap();

    let confirm_nickname = read_password_from_tty(Some("Confirm account nickname: ")).unwrap();
    if !(confirm_nickname == nickname) {
        eprintln!("nicknames don't match");
        process::exit(1);
    }

    let confirm_password = read_password_from_tty(Some("Confirm master password:")).unwrap();
    if !(confirm_password == password) {
        eprintln!("passwords don't match");
        process::exit(1);
    }

    let hash = oplop_hash(&nickname, &password);
    let password = oplop_password(&hash);
    set_clipboard(&password);
}

pub fn run(args: &[String]) {
    if args.len() == 0 {
        oplop();
    } else if args.len() == 1 && args[0] == "new" {
        oplop_new();
    } else {
        print_usage();
    }
}

#[cfg(test)]
mod tests {
    use super::{oplop_hash, oplop_password};
    use json;
    use std::fs;

    #[test]
    fn example() {
        let cases = fs::read_to_string("tests/support/testdata.json").unwrap();
        let parsed = json::parse(cases.as_str()).unwrap();
        for case in parsed.members() {
            let label = &case["label"].as_str().unwrap();
            let master = &case["master"].as_str().unwrap();
            let hash = &case["hash"].as_str().unwrap();
            let password = &case["password"].as_str().unwrap();
            assert_eq!(&oplop_hash(label, master), hash);
            assert_eq!(&oplop_password(hash), password);
        }
    }
}
