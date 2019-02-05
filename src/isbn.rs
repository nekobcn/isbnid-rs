//! isbnid-rs
//! Rust ISBN identifier library
//!
//! isbnid is a simple crate to handle ISBN identification numbers.
//! isbnid will store, check and convert ISBNs in ISBN10, and ISBN13
//! formats and it will transform between them and output in URN form.
//! isbnid can also output ISBN numbers with the correct hyphens
//! corresponding to the actual issuance authorities. The information
//! is retrieved from https://www.isbn-international.org/. ISBN numbers
//! have a complex internal structure which roughly represents the country,
//! the language and the publisher. See also https://en.wikipedia.org/wiki/ISBN.
//!
//! # Install
//! You can find this crate on crates.io and it can be used by adding isbnid to the
//! dependencies in your project's Cargo.toml and root file.
//!
//! ```text
//! [dependencies]
//! isbnid = "0.1.*"
//! ```
//!
//! # Usage
//!
//! ```text
//! extern crate isbnid;
//!
//! use isbnid::isbn;
//!
//! let id = isbn::ISBN("9780553109535").unwrap();
//! println!("{}", id.isbn10());
//! println!("{}", id.isbn13());
//! ```
//!
//! ```text
//! 0553109537
//! 9780553109535
//! ```


use std::result;
use std::str::FromStr;

use regex::Regex;
use hyphen;


#[derive(Debug)]
pub enum ISBNError {
    /// String doesn't form a valid ISBN10 or ISBN13 number encoding
    Format,
    /// ISBN Check digit is not valid
    CheckDigit,
    /// ISBN13 Bookland encoding (EAN-13) is different form 978 or 979,
    /// or it is 979 when converting to ISBN10
    Bookland,
    /// ISBN doesn't belong to the ISBN International official range
    Range
}

fn digit10(id: &str) -> u64 {
    let mut n = u64::from_str(&id[0..9]).unwrap();
    let mut d = 0u64;

    for i in 1..10 {
        d = d + (10 - i) * (n % 10);
        n = n / 10;
    }
    d % 11
}

fn digit13(id: &str) -> u64 {
    let mut n = u64::from_str(&id[0..12]).unwrap();
    let mut d = 0u64;

    for i in 1..13 {
        d = d + (1 + 2 * (i % 2)) * (n % 10);
        n = n / 10;
    }
    // Kludge for unsigned negative module
    (100000000000000000u64 - d) % 10 // 10^17
}

pub struct ISBN {
    id: String,
}

impl ISBN {
    /// Creates a new ISBN number object.
    /// It will fail if the encoding is incorrect or if the Bookland is not 978, 979
    pub fn new(id: &str) -> result::Result<ISBN, ISBNError> {
        let reif = Regex::new(r"^(\d(-| )?){9}(x|X|\d|(\d(-| )?){3}\d)$").unwrap();
        let reis = Regex::new(r"[^0-9X]").unwrap();

        if ! id.is_ascii() || ! reif.is_match(id) {
            // Invalid ISBN format
            return Err(ISBNError::Format)
        }
        let nid: String = reis.replace_all(&id.to_uppercase(), "").into();
        if nid.len() == 13 {
            if &nid[0..3] != "978" && &nid[0..3] != "979" {
                // Invalid Bookland code
                return Err(ISBNError::Bookland);
            }
            if u64::from_str(&nid[12..13]).unwrap() != digit13(&nid) {
                // Invalid ISBN check digit
                return Err(ISBNError::CheckDigit);
            }
            return Ok(ISBN{id: nid});
        }
        if nid.len() == 10 {
            let id13 = "978".to_string() + &nid[0..9];
            if &nid[9..10] == "X" && 10 != digit10(&nid) {
                 // Invalid ISBN check digit
                return Err(ISBNError::CheckDigit);
            }
            if &nid[9..10] == "X" && 10 == digit10(&nid) {
                return Ok(ISBN{id: format!("{}{}", &id13, digit13(&id13))});
            }
            if u64::from_str(&nid[9..10]).unwrap() != digit10(&nid) {
                // Invalid ISBN check digit
                return Err(ISBNError::CheckDigit);
            }
            return Ok(ISBN{id: format!("{}{}", &id13, digit13(&id13))});
        }
        // Invalid ISBN format, dead code by regex
        assert!(false);
        Err(ISBNError::Format)
    }

    /// Returns the ISBN10 encoding. It will fail if the ISBN13 Bookland is 979
    /// as ISBN10 is only defined for 978
    pub fn isbn10(&self) -> Result<String, ISBNError> {
        if &self.id[0..3] != "978" {
            // Invalid Bookland code
            return Err(ISBNError::Bookland)
        }
        let check10 = digit10(&self.id[3..12]);
        if check10 == 10 {
            Ok(format!("{}X", &self.id[3..12] ))
        }
        else {
            Ok(format!("{}{}", &self.id[3..12], check10))
        }
    }

    /// Returns the ISBN13 encoding. The internal encoding is ISBN13 so this will never fail
    pub fn isbn13(&self) -> String {
        format!("{}", &self.id)
    }

    /// Returns a hyphenated ISBN13 number. It will fail if the ISBN number is not registered
    pub fn hyphen(&self) -> Result<String, ISBNError> {
        let (grp, reg, pbl) = hyphen::segments(&self.id);
        if grp == 0 {
            return Err(ISBNError::Range);
        }
        Ok([&self.id[0..3], &self.id[3..3 + grp],  &self.id[3 + grp .. 3 + grp + reg], &self.id[12 - pbl..12], &self.id[12..13]].join("-"))
    }

    /// RFC 2888, URN Encoding of ISBN. https://www.ietf.org/rfc/rfc2288
    pub fn urn(&self) -> String {
        format!("URN:ISBN:{}", &self.id)
    }

    /// Returns doi formated ISBN. It fail if the ISBN number is not registered
    pub fn doi(&self) -> Result<String, ISBNError> {
        let (grp, reg, pbl) = hyphen::segments(&self.id);
        if grp == 0 {
            return Err(ISBNError::Range);
        }
        Ok(format!("10.{}.{}/{}", &self.id[0..3], &self.id[3..3 + grp + reg], &self.id[12 - pbl..13]))
    }

    /// Static ISBN format validation
    pub fn is_valid(id: &str) -> bool {
        match ISBN::new(id) {
            Ok(_) => true,
            Err(_) => false
        }
    }
}
