extern crate isbnid;

use isbnid::isbn;

/*       
    "9780130091130"
    "1484200772"
    "123456789X"
*/

static ISBNTUP: [(&'static str, [&'static str; 5]); 5] = [
    ("012345672X",      ["012345672X", "9780123456724", "978-0-12-345672-4", "URN:ISBN:9780123456724", "10.978.012/3456724"]),
    ("9780387308869",   ["0387308865", "9780387308869", "978-0-387-30886-9", "URN:ISBN:9780387308869", "10.978.0387/308869"]),
    ("9780393334777",   ["0393334775", "9780393334777", "978-0-393-33477-7", "URN:ISBN:9780393334777", "10.978.0393/334777"]),
    ("9781593273880",   ["1593273886", "9781593273880", "978-1-59327-388-0", "URN:ISBN:9781593273880", "10.978.159327/3880"]),
    ("9788478447749",   ["8478447741", "9788478447749", "978-84-7844-774-9", "URN:ISBN:9788478447749", "10.978.847844/7749"])
];


#[test]
fn test_out() {
    for tup in &ISBNTUP {
        let io = isbn::ISBN::new(tup.0).unwrap();        
        assert_eq!(io.isbn10().unwrap(), tup.1[0]);
        assert_eq!(io.isbn13(), tup.1[1]);
    }
}

#[test]
fn test_hyphen() {
    for tup in &ISBNTUP {
        let io = isbn::ISBN::new(tup.0).unwrap();
        assert!(io.hyphen().unwrap() == tup.1[2]);
    }
}

#[test]
#[should_panic]
fn test_hyphen_reg() {
    let io = isbn::ISBN::new("9799999999990").unwrap();
    println!("{}", io.hyphen().unwrap());
}



#[test]
fn test_urn() {
    for tup in &ISBNTUP {
        let io = isbn::ISBN::new(tup.0).unwrap();
        assert_eq!(io.urn(), tup.1[3]);
    }   
}

#[test]
fn test_doi() {
    for tup in &ISBNTUP {
        let io = isbn::ISBN::new(tup.0).unwrap();
        assert_eq!(io.doi().unwrap(), tup.1[4]);
    }   
}