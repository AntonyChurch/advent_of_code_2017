extern crate adventofcode;

use adventofcode::four;

#[test]
fn example_a_one(){        
    assert_eq!(true, four::is_passphrase_valid_a(String::from("aa bb cc dd ee")));
}

#[test]
fn example_a_two(){        
    assert_eq!(false, four::is_passphrase_valid_a(String::from("aa bb cc dd aa")));
}

#[test]
fn example_a_three(){        
    assert_eq!(true, four::is_passphrase_valid_a(String::from("aa bb cc dd aaa")));
}

#[test]
fn question_a(){
    assert_eq!(455, four::four_a());
}

#[test]
fn example_b_one(){
    assert_eq!(true, four::is_passphrase_valid_b(String::from("abcde fghij")));
}

#[test]
fn example_b_two(){
    assert_eq!(false, four::is_passphrase_valid_b(String::from("abcde xyz ecdab")));
}

#[test]
fn example_b_three(){
    assert_eq!(true, four::is_passphrase_valid_b(String::from("a ab abc abd abf abj")));
}

#[test]
fn example_b_four(){
    assert_eq!(true, four::is_passphrase_valid_b(String::from("iiii oiii ooii oooi oooo")));
}

#[test]
fn example_b_five(){
    assert_eq!(true, four::is_passphrase_valid_b(String::from("oiii ioii iioi iiio")));
}