extern crate rpassword;

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

fn oplop() {
    println!("normal oplop");
}

fn oplop_new() {
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
