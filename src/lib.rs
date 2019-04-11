use md5;
use std::process;

extern crate rpassword;
extern crate clipboard;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;

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


fn set_clipboard(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.to_owned()).expect("Error copying to clipboard.");
    println!("Account password copied to clipboard!");
}

fn oplop() {
    let nickname = rpassword::read_password_from_tty(Some("Enter account nickname: ")).unwrap();
    let password = rpassword::read_password_from_tty(Some("Enter master password:")).unwrap();
    let hash = format!("oplop{}{}", nickname, password);
    set_clipboard(&hash);
}

fn oplop_new() {
    let nickname = rpassword::read_password_from_tty(Some("Enter account nickname: ")).unwrap();
    let password = rpassword::read_password_from_tty(Some("Enter master password:")).unwrap();

    let confirm_nickname = rpassword::read_password_from_tty(Some("Confirm account nickname: ")).unwrap();
    if !(confirm_nickname == nickname) {
        eprintln!("nicknames don't match");
        process::exit(1);
    }

    let confirm_password = rpassword::read_password_from_tty(Some("Confirm master password:")).unwrap();
    if !(confirm_nickname == nickname) {
        eprintln!("nicknames don't match");
        process::exit(1);
    }

    let hash = format!("oplop{}{}", nickname, password);
    set_clipboard(&hash);
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
    use super::*;

    #[test]
    fn example() {
        assert_eq!(
            (1 == 1),
            true
        );
    }
}
