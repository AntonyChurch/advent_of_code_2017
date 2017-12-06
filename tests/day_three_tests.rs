extern crate adventofcode;

#[test]
fn example_a_one(){
    assert_eq!(0, adventofcode::three_a(1));
}

#[test]
fn example_a_two(){
    assert_eq!(3, adventofcode::three_a(12));
}

#[test]
fn example_a_three(){
    assert_eq!(2, adventofcode::three_a(23));
}

#[test]
fn example_a_four(){
    assert_eq!(31, adventofcode::three_a(1024));
}