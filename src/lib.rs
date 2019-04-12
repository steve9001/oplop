#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use std::process;

use base64;
use clipboard::ClipboardProvider;
use clipboard::ClipboardContext;
use md5;
use rpassword::read_password_from_tty;
use hex;

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

use std::str;
fn oplop_hash(label: &str, master: &str) -> String {
    //let foo = b'\\xbbd\\x9c\\x83\\xdd\\x1e\\xa5\\xc9\\xd9\\xde\\xc9\\xa1\\x8d\\xf0\\xff\\xe9';
    //println!("foo {}", foo);
    let input = format!("{}{}", master, label);
    println!("input: {}", input);
    //let digest = md5::compute(format!("{:b}{:b}", master, label));
    let digest = md5::compute(input);
    let fdigest = format!("{:?}", *digest);
    println!("fdigest: {}", fdigest);
    //let digest2 = hex::decode(*digest).unwrap();
    //println!("digest2: {:x}", digest2);

    //let digest2a = String::from_utf8((&*digest).to_vec());
    //println!("digest2a: {:?}", digest2a);

    //let vec_of_chars: Vec<char> = *digest.iter().map(|byte: &u8| *byte as char).collect();
    //println!("vec_of_chars: {:?}", vec_of_chars);


    let d = [65, 36, 188, 10, 147, 53, 194, 127, 8, 111, 36, 186, 32, 122, 73, 18];

    let vec_of_chars: Vec<char> = d.iter().map(|byte: &u8| *byte as char).collect();
    println!("vec_of_chars: {:?}", vec_of_chars);
    let s: String = vec_of_chars.iter().collect();
    println!("vec_of_chars_s: {:?}", s);
    let se = base64::encode_config(&s, base64::URL_SAFE);
    println!("vec_of_chars se: {}", se);



    //let digest3 = str::from_utf8(&*digest).unwrap();
    //println!("digest3: {:?}", digest3);
    //let ddigest = format!("{:?}", digest);
    //println!("ddigest: {}", ddigest);
    //let digest = base64::encode_config(&format!("{:?}", digest), base64::URL_SAFE);
    //println!("b64digest: {}", ddigest);
    String::from(format!("{:?}", digest))
    //String::from("QSS8CpM1wn8IbyS6IHpJEg==")
}

fn oplop_password(hash: &str) -> String {
    String::from("QSS8CpM1")
}

fn set_clipboard(text: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(text.to_owned()).expect("Error copying to clipboard.");
    println!("Account password copied to clipboard!");
}

fn oplop() {
    let nickname = read_password_from_tty(Some("Enter account nickname: ")).unwrap();
    let password = read_password_from_tty(Some("Enter master password:")).unwrap();
    let hash = format!("oplop{}{}", nickname, password);
    set_clipboard(&hash);
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
    use super::oplop_hash;
    use std::fs;
    use json;

    #[test]
    fn example() {
        let cases = fs::read_to_string("tests/support/testdata.json").unwrap();
        let parsed = json::parse(cases.as_str()).unwrap();
        for case in parsed.members() {
            let label = &case["label"].as_str().unwrap();
            let master = &case["master"].as_str().unwrap();
            let hash = &case["hash"].as_str().unwrap();
            assert_eq!(
                &oplop_hash(label, master),
                hash
            );
            break
        }
    }
}
