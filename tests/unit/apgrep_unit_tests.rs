use super::*;

#[test]
fn should_find_one() {
    let query = "\
    Rust:
    safe, fast and productive.
    Pick three
    ";
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
}