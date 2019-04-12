use std::process;

use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use rpassword::read_password_from_tty;
mod algorithm;

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
    let nickname = read_password_from_tty(Some("Enter account nickname: ")).unwrap();
    let password = read_password_from_tty(Some("Enter master password:")).unwrap();
    let password = algorithm::get_password(&nickname, &password);
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

    let password = algorithm::get_password(&nickname, &password);
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
