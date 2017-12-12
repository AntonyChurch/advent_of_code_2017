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

#[test]
fn example_b_one(){
    assert_eq!(2, adventofcode::three_b(1));
}

#[test]
fn example_b_two(){
    assert_eq!(4, adventofcode::three_b(2));
}

#[test]
fn example_b_three(){
    assert_eq!(4, adventofcode::three_b(3));
}

#[test]
fn example_b_four(){
    assert_eq!(5, adventofcode::three_b(4));
}

#[test]
fn example_b_five(){
    assert_eq!(10, adventofcode::three_b(5));
}