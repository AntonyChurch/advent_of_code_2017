pub fn one_a(input: String) -> u32 {
    let input_ints = get_int_list_from_string(input);
    let mut to_sum: Vec<u32> = Vec::new();

    let mut i = 0;

    loop {
        if i > input_ints.len() {
            break;
        }

        if i == input_ints.len() - 1 {
            if input_ints[i] == input_ints[0]{
                to_sum.push(input_ints[i]);
            }

            break;
        }

        if input_ints[i] == input_ints[i + 1] {
            to_sum.push(input_ints[i]);
        }

        i = i + 1;        
    }

    let mut sum: u32 = 0;

    for x in to_sum.iter() {
        sum += x;
    }

    return sum;
}

fn get_int_list_from_string(input: String) -> Vec<u32> {
    let mut list: Vec<u32> = Vec::new();

    for c in input.chars() {
        let st = c.to_string();
        let int = st.parse::<u32>().unwrap();
        list.push(int);
    }

    return list;
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