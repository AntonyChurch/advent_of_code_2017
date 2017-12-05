extern crate adventofcode;

#[test]
fn example_a_one(){        
    assert_eq!(3, adventofcode::one_a(String::from("1122")));
}

#[test]
fn example_a_two(){
    assert_eq!(4, adventofcode::one_a(String::from("1111")));
}

#[test]
fn example_a_three(){
    assert_eq!(0, adventofcode::one_a(String::from("1234")));
}

#[test]
fn example_a_four(){
    assert_eq!(9, adventofcode::one_a(String::from("91212129")));
}

#[test]
fn exmaple_b_one(){
    assert_eq!(6, adventofcode::one_b(String::from("1212")));
}

#[test]
fn exmaple_b_two(){
    assert_eq!(0, adventofcode::one_b(String::from("1221")));
}

#[test]
fn exmaple_b_three(){
    assert_eq!(4, adventofcode::one_b(String::from("123425")));
}

#[test]
fn exmaple_b_four(){
    assert_eq!(12, adventofcode::one_b(String::from("123123")));
}

#[test]
fn exmaple_b_five(){
    assert_eq!(4, adventofcode::one_b(String::from("12131415")));
}