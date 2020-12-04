use std::collections::HashMap;
use std::io::{self, BufRead};

pub struct Passport(HashMap<String, String>);

impl From<Vec<String>> for Passport {
    fn from(raw: Vec<String>) -> Passport {
        let mut map = HashMap::new();

        for line in raw {
            for part in line.split_whitespace() {
                let mut entry = part.split(":");
                let field = entry.next().expect("has field name").to_owned();
                let value = entry.next().expect("has field value").to_owned();
                map.insert(field, value);
            }
        }

        Passport(map)
    }
}

impl Passport {
    fn get(&self, field: &str) -> Option<&String> {
        self.0.get(field)
    }
}

fn is_valid_year(value: &str, min: u32, max: u32) -> bool {
    if value.len() != 4 {
        return false;
    }

    match value.parse::<u32>() {
        Err(_) => false,
        Ok(v) if v < min || v > max => false,
        _ => true,
    }
}

fn is_valid_byr(value: Option<&str>) -> bool {
    match value {
        None => false,
        Some(v) => is_valid_year(v, 1920, 2002),
    }
}

fn is_valid_iyr(value: Option<&str>) -> bool {
    match value {
        None => false,
        Some(v) => is_valid_year(v, 2010, 2020),
    }
}

fn is_valid_eyr(value: Option<&str>) -> bool {
    match value {
        None => false,
        Some(v) => is_valid_year(v, 2020, 2030),
    }
}

fn is_valid_hgt(value: Option<&str>) -> bool {
    let v = match value {
        None => return false,
        Some(v) => v,
    };

    let chars: Vec<_> = v.chars().collect();

    match &chars[..] {
        [_, _, _, 'c', 'm'] => match v[0..3].parse::<u8>() {
            Err(_) => false,
            Ok(h) if h < 150 || h > 193 => false,
            _ => true,
        },
        [_, _, 'i', 'n'] => match v[0..2].parse::<u8>() {
            Err(_) => false,
            Ok(h) if h < 59 || h > 76 => false,
            _ => true,
        },
        _ => false,
    }
}

fn is_valid_hcl(value: Option<&str>) -> bool {
    match value {
        None => false,
        Some(v) => {
            let chars: Vec<_> = v.chars().collect();

            if chars.len() != 7 {
                return false;
            }

            if chars[0] != '#' {
                return false;
            }

            if !chars[1..].iter().all(|ch| ch.is_digit(16)) {
                return false;
            }

            true
        }
    }
}

fn is_valid_ecl(value: Option<&str>) -> bool {
    match value {
        None => false,
        Some("amb") | Some("blu") | Some("brn") | Some("gry") | Some("grn") | Some("hzl")
        | Some("oth") => true,
        _ => false,
    }
}

fn is_valid_pid(value: Option<&str>) -> bool {
    match value {
        None => false,
        Some(v) => {
            let chars: Vec<_> = v.chars().collect();

            if chars.len() != 9 {
                return false;
            }

            if !chars.iter().all(|ch| ch.is_digit(10)) {
                return false;
            }

            true
        }
    }
}

fn is_valid_cid(_value: Option<&str>) -> bool {
    true
}

fn is_valid_1(passport: &Passport) -> bool {
    passport.get("byr").is_some()
        && passport.get("iyr").is_some()
        && passport.get("eyr").is_some()
        && passport.get("hgt").is_some()
        && passport.get("hcl").is_some()
        && passport.get("ecl").is_some()
        && passport.get("pid").is_some()
}

fn is_valid_2(passport: &Passport) -> bool {
    is_valid_byr(passport.get("byr").map(|s| &s[..]))
        && is_valid_iyr(passport.get("iyr").map(|s| &s[..]))
        && is_valid_eyr(passport.get("eyr").map(|s| &s[..]))
        && is_valid_hgt(passport.get("hgt").map(|s| &s[..]))
        && is_valid_hcl(passport.get("hcl").map(|s| &s[..]))
        && is_valid_ecl(passport.get("ecl").map(|s| &s[..]))
        && is_valid_pid(passport.get("pid").map(|s| &s[..]))
        && is_valid_cid(passport.get("cid").map(|s| &s[..]))
}

fn main() {
    let stdin = io::stdin();

    let mut count_1 = 0;
    let mut count_2 = 0;

    let mut lines = stdin.lock().lines().map(|l| l.unwrap());

    loop {
        let raw = lines.by_ref().take_while(|l| l != "").collect::<Vec<_>>();

        if raw.is_empty() {
            break;
        }

        let passport = Passport::from(raw);

        if is_valid_1(&passport) {
            count_1 += 1;
        }

        if is_valid_2(&passport) {
            count_2 += 1;
        }
    }

    println!("problem #1 = {:?}", count_1);
    println!("problem #2 = {:?}", count_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_byr() {
        assert_eq!(is_valid_byr(Some("03")), false);
        assert_eq!(is_valid_byr(Some("1919")), false);
        assert_eq!(is_valid_byr(Some("1920")), true);
        assert_eq!(is_valid_byr(Some("2002")), true);
        assert_eq!(is_valid_byr(Some("2003")), false);
        assert_eq!(is_valid_byr(Some("02002")), false);
        assert_eq!(is_valid_byr(Some("2O2O")), false);
        assert_eq!(is_valid_byr(None), false);
    }

    #[test]
    fn test_is_valid_iyr() {
        assert_eq!(is_valid_iyr(Some("03")), false);
        assert_eq!(is_valid_iyr(Some("2009")), false);
        assert_eq!(is_valid_iyr(Some("2010")), true);
        assert_eq!(is_valid_iyr(Some("2020")), true);
        assert_eq!(is_valid_iyr(Some("2021")), false);
        assert_eq!(is_valid_iyr(Some("02020")), false);
        assert_eq!(is_valid_iyr(Some("2O2O")), false);
        assert_eq!(is_valid_iyr(None), false);
    }

    #[test]
    fn test_is_valid_eyr() {
        assert_eq!(is_valid_eyr(Some("03")), false);
        assert_eq!(is_valid_eyr(Some("2019")), false);
        assert_eq!(is_valid_eyr(Some("2020")), true);
        assert_eq!(is_valid_eyr(Some("2030")), true);
        assert_eq!(is_valid_eyr(Some("2031")), false);
        assert_eq!(is_valid_eyr(Some("02030")), false);
        assert_eq!(is_valid_eyr(Some("2O2O")), false);
        assert_eq!(is_valid_eyr(None), false);
    }

    #[test]
    fn test_is_valid_hgt() {
        assert_eq!(is_valid_hgt(Some("149cm")), false);
        assert_eq!(is_valid_hgt(Some("150cm")), true);
        assert_eq!(is_valid_hgt(Some("193cm")), true);
        assert_eq!(is_valid_hgt(Some("194cm")), false);
        assert_eq!(is_valid_hgt(Some("150c")), false);
        assert_eq!(is_valid_hgt(Some("cm")), false);

        assert_eq!(is_valid_hgt(Some("58in")), false);
        assert_eq!(is_valid_hgt(Some("59in")), true);
        assert_eq!(is_valid_hgt(Some("76in")), true);
        assert_eq!(is_valid_hgt(Some("77in")), false);
        assert_eq!(is_valid_hgt(Some("76i")), false);
        assert_eq!(is_valid_hgt(Some("in")), false);
        assert_eq!(is_valid_hgt(None), false);
    }

    #[test]
    fn test_is_valid_hcl() {
        assert_eq!(is_valid_hcl(Some("#000000")), true);
        assert_eq!(is_valid_hcl(Some("#ffffff")), true);
        assert_eq!(is_valid_hcl(Some("#123abc")), true);

        assert_eq!(is_valid_hcl(Some("#123abz")), false);
        assert_eq!(is_valid_hcl(Some("123abc")), false);
        assert_eq!(is_valid_hcl(Some("#12345")), false);
    }

    #[test]
    fn test_is_valid_ecl() {
        assert_eq!(is_valid_ecl(Some("amb")), true);
        assert_eq!(is_valid_ecl(Some("blu")), true);
        assert_eq!(is_valid_ecl(Some("brn")), true);
        assert_eq!(is_valid_ecl(Some("gry")), true);
        assert_eq!(is_valid_ecl(Some("grn")), true);
        assert_eq!(is_valid_ecl(Some("hzl")), true);
        assert_eq!(is_valid_ecl(Some("oth")), true);

        assert_eq!(is_valid_ecl(Some("wat")), false);
        assert_eq!(is_valid_ecl(None), false);
    }

    #[test]
    fn test_is_valid_pid() {
        assert_eq!(is_valid_pid(Some("000000001")), true);
        assert_eq!(is_valid_pid(Some("999999999")), true);

        assert_eq!(is_valid_pid(Some("0")), false);
        assert_eq!(is_valid_pid(Some("09")), false);
        assert_eq!(is_valid_pid(Some("0123456789")), false);
        assert_eq!(is_valid_pid(None), false);
    }
}
