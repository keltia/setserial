
use std::fs;

use anyhow::{bail, Result};
use chrono::prelude::*;
use regex::Regex;

/// Check the format of the serial and return two separate strings for date and serial
///
fn parse_serial(input: &str) -> Result<(String, String)> {
    let r = Regex::new(r##"(20[0-3][0-9][01][0-9][0-3][0-9])(\d\d)"##).unwrap();
    match r.captures(input) {
        Some(c)  => {
            Ok((c[1].to_owned(), c[2].to_owned()))
        },
        _ => bail!("invalid format")
    }
}

/// Process the current serial:
/// - if serial from today, increment the last two digits
/// - if not, replace it with the first one from today
///
fn process_one(s: &str) -> Result<String> {
    let s: Vec<&str> = s.split('\n').collect();
    let s = s[0].to_owned();
    let (s_date, s_serial) = parse_serial(&s)?;

    let date = NaiveDate::parse_from_str(&s_date, "%Y%m%d")?;
    let t = Local::now();
    let t = NaiveDate::from_ymd_opt(t.year(), t.month(), t.day()).unwrap();
    let serial = if t == date {
        let serial = s_serial.parse::<usize>()? + 1;
        format!("{serial:02}")
    } else {
        "01".to_owned()
    };
    Ok(format!("{}{}", t.format("%Y%m%d"), serial))
}

/// Read the current serial file and replace it with a new serial for today.
/// Old serial is in .old file.
///
fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("filename not given");
        std::process::exit(1);
    }
    let fname = args[1].to_owned();
    let content = fs::read_to_string(&fname)?;
    let out = process_one(&content).unwrap();
    fs::rename(&fname, format!("{fname}.old"))?;
    fs::write(&fname, out)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_serial() {
        let input = fs::read_to_string("testdata/serial.txt").unwrap();

        dbg!(&input);
        let (d, s) = parse_serial(&input).unwrap();
        assert_eq!("20010527", d);
        assert_eq!("42", s);
    }

    #[test]
    fn test_process_one_old() {
        let t = Local::now();
        let t = NaiveDate::from_ymd_opt(t.year(), t.month(), t.day()).unwrap();
        let t = t.format("%Y%m%d").to_string();
        let t = format!("{t}01");

        let input = fs::read_to_string("testdata/serial.txt").unwrap();
        if let Ok(res) = process_one(&input) {
            assert_eq!(t, res)
        }
    }

    #[test]
    fn test_process_one_now() {
        let t = Local::now();
        let t = NaiveDate::from_ymd_opt(t.year(), t.month(), t.day()).unwrap();
        let t = t.format("%Y%m%d").to_string();
        let t2 = format!("{t}02");

        let input = t;
        if let Ok(res) = process_one(&input) {
            assert_eq!(t2, res)
        }
    }
}