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

pub fn run(args: &[String]) {
    if args.len() == 0 {
        println!("normal oplop");
    } else if args.len() == 1 && args[0] == "new" {
        println!("new oplop");
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
