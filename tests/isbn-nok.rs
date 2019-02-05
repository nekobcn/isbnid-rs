extern crate isbnid;

use isbnid::isbn;

/*       
    "9780130091130"
    "1484200772"
    "123456789X"
*/

#[test]
fn test_in() {
    let isbntup = [ "X123456781", "012345678X", "9780123456780", "9780123456781", "9790123456780", "9790123456781", "9890123456781"];
    for tup in &isbntup {
        assert!(! isbn::ISBN::is_valid(tup));
    }    
}