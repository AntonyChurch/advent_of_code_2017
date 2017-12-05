use std::collections::LinkedList;

fn one_a(input: String) -> u32 {
    let to_sum: LinkedList<u32> = LinkedList::new();

    let i = 0;

    loop {
        if i > input.len() {
            break;
        }

        if i == 0 {
            continue;
        }

        if i == input.len() - 1 {
            if input.chars().nth(i) == input.chars().nth(0) {
                to_sum.push_back(input.chars().nth(i).to_digit(10));
            }

            continue;
        }

        if input.chars().nth(i) == input.chars().nth(i + 1) {
            to_sum.push_back(input.chars().nth(i).to_digit(10));
        }

        i = i + 1;        
    }

    let mut sum: u32 = 0;

    for x in to_sum.iter() {
        sum += x;
    }

    return sum;
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