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
}

fn oplop() {
    let nickname = rpassword::read_password_from_tty(Some("Enter account nickname: ")).unwrap();
    let password = rpassword::read_password_from_tty(Some("Enter master password:")).unwrap();
    let hash = format!("oplop{}{}", nickname, password);
    set_clipboard(&hash);
}

fn oplop_new() {
    //request nickname
    //request master password
    //confirm nickname
    //confirm master password
    set_clipboard("new oplop");
    println!("new oplop");
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
