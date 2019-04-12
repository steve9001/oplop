use base64;
use md5;
use regex::Regex;

pub fn get_password(nickname: &str, password: &str) -> String {
    oplop_password(&oplop_hash(&nickname, &password))
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
                m.as_str().to_owned() + &hash[..8-prefix_len]
            }
        }
        None => "1".to_owned() + &hash[..7]
    }
}

#[cfg(test)]
mod tests {
    use super::{oplop_hash, oplop_password};
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
            let password = &case["password"].as_str().unwrap();
            assert_eq!(
                &oplop_hash(label, master),
                hash
            );
            assert_eq!(
                &oplop_password(hash),
                password
            );
        }
    }
}
