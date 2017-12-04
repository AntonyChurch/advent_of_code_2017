fn one_a(input: String) -> u32 {
    // let string_size = |x: String| -> usize { x.chars().count() };
    // let Input_Len: usize = input.len();
    // const Input_Length = Input_Len;

    // let to_sum: [i32; Input_Len] = [0; Input_Len];

    // for c in input.chars() {

    // }

    if input == "" {            
    }

    println!("{}", input);
    return 0;
}

#[test]
fn example_one(){        
    assert_eq!(3, one_a(String::from("1122")));
}

#[test]
fn example_two(){
    assert_eq!(4, one_a(String::from("1111")));
}

#[test]
fn example_three(){
    assert_eq!(0, one_a(String::from("1234")));
}

#[test]
fn example_four(){
    assert_eq!(9, one_a(String::from("91212129")));
}

// #[cfg(test)]
// mod day_one_tests {
//     #[test]
//     fn example_one(){        
//         assert_eq!(3, one_a(String::from("1122")));
//     }

//     #[test]
//     fn example_two(){
//         assert_eq!(4, one_a(String::from("1111")));
//     }

//     #[test]
//     fn example_three(){
//         assert_eq!(0, one_a(String::from("1234")));
//     }

//     #[test]
//     fn example_four(){
//         assert_eq!(9, one_a(String::from("91212129")));
//     }
// }