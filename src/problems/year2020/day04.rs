use crate::{Error, Solution};

day!(Day04, 2020, 4, "Passport Processing");

impl Solution for Day04 {
    fn part1(&self, input: &str) -> Result<String, Error> {
        let passports = parse_input(input);
        Ok(passports
            .iter()
            .filter(|p| p.is_valid1())
            .count()
            .to_string())
    }

    fn part2(&self, input: &str) -> Result<String, Error> {
        let passports = parse_input(input);
        Ok(passports
            .iter()
            .filter(|p| p.is_valid2())
            .count()
            .to_string())
    }
}

fn parse_input<'a>(input: &'a str) -> Vec<Passport<'a>> {
    let input_lines = input.trim().lines();
    let mut passports = Vec::new();

    let mut passport = Passport::new();
    for line in input_lines {
        if line.is_empty() {
            passports.push(passport);
            passport = Passport::new();
        } else {
            for element in line.split_whitespace() {
                let mut element = element.split(':');
                let key = element.next().unwrap();
                let value = element.next().unwrap();

                match key {
                    "byr" => passport.byr = Some(value.parse().unwrap()),
                    "iyr" => passport.iyr = Some(value.parse().unwrap()),
                    "eyr" => passport.eyr = Some(value.parse().unwrap()),
                    "hgt" => passport.hgt = Some(value),
                    "hcl" => passport.hcl = Some(value),
                    "ecl" => passport.ecl = Some(value),
                    "pid" => passport.pid = Some(value),
                    "cid" => passport.cid = Some(value),
                    _ => panic!("Invalid key"),
                }
            }
        }
    }
    passports.push(passport);

    passports
}

struct Passport<'a> {
    byr: Option<u16>,
    iyr: Option<u16>,
    eyr: Option<u16>,
    hgt: Option<&'a str>,
    hcl: Option<&'a str>,
    ecl: Option<&'a str>,
    pid: Option<&'a str>,
    cid: Option<&'a str>,
}
impl Passport<'_> {
    fn new() -> Self {
        Self {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
        }
    }

    fn is_valid1(&self) -> bool {
        self.byr.is_some()
            && self.iyr.is_some()
            && self.eyr.is_some()
            && self.hgt.is_some()
            && self.hcl.is_some()
            && self.ecl.is_some()
            && self.pid.is_some()
    }

    fn is_valid2(&self) -> bool {
        // byr (Birth Year) - four digits; at least 1920 and at most 2002.
        if let Some(byr) = self.byr {
            if !(1920..=2002).contains(&byr) {
                return false;
            }
        } else {
            return false;
        }

        // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
        if let Some(iyr) = self.iyr {
            if !(2010..=2020).contains(&iyr) {
                return false;
            }
        } else {
            return false;
        }

        // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
        if let Some(eyr) = self.eyr {
            if !(2020..=2030).contains(&eyr) {
                return false;
            }
        } else {
            return false;
        }

        // hgt (Height) - a number followed by either cm or in:
        //     If cm, the number must be at least 150 and at most 193.
        //     If in, the number must be at least 59 and at most 76.
        if let Some(hgt) = self.hgt {
            if hgt.ends_with("cm") {
                let hgt: i32 = hgt.trim_end_matches("cm").parse().unwrap();
                if !(150..=193).contains(&hgt) {
                    return false;
                }
            } else if hgt.ends_with("in") {
                let hgt: i32 = hgt.trim_end_matches("in").parse().unwrap();
                if !(59..=76).contains(&hgt) {
                    return false;
                }
            } else {
                return false;
            }
        } else {
            return false;
        }

        // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
        if let Some(hcl) = self.hcl {
            if !hcl.starts_with('#') {
                return false;
            }
            let hcl = hcl.trim_start_matches('#');
            if hcl.chars().count() != 6 {
                return false;
            }
            if hcl.chars().any(|c| !c.is_ascii_hexdigit()) {
                return false;
            }
        } else {
            return false;
        }

        // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
        if let Some(ecl) = self.ecl {
            if !["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&ecl) {
                return false;
            }
        } else {
            return false;
        }

        // pid (Passport ID) - a nine-digit number, including leading zeroes.
        if let Some(pid) = self.pid {
            if pid.chars().count() != 9 {
                return false;
            }
            if pid.chars().any(|c| !c.is_ascii_digit()) {
                return false;
            }
        } else {
            return false;
        }

        true
    }
}
