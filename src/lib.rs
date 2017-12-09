pub fn one_a(input: String) -> u32 {
    let input_ints = get_int_list_from_string(input);
    let mut to_sum: Vec<u32> = Vec::new();

    let mut i = 0;

    loop {
        if i >= input_ints.len() {
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

    return sum_int_vec(to_sum);
}

pub fn one_b(input: String) -> u32 {
    let input_ints = get_int_list_from_string(input);
    let mut to_sum: Vec<u32> = Vec::new();

    let mut split_int1 = input_ints.clone();
    let split_int2 = split_int1.split_off(input_ints.len() / 2);

    let mut combined_ints: Vec<u32> = Vec::new();
    for int in split_int2.iter() {
        let x: u32 = *int;
        combined_ints.push(x);
    }

    for int in split_int1.iter() {
        let x: u32 = *int;
        combined_ints.push(x);
    }

    let mut i = 0;

    loop {
        if i >= input_ints.len() {
            break;
        }

        if input_ints[i] == combined_ints[i] {
            to_sum.push(input_ints[i]);
        }

        i = i + 1;
    }

    return sum_int_vec(to_sum);
}

pub fn two_a(input: Vec<Vec<u32>>) -> u32 {
    let mut to_sum: Vec<u32> = Vec::new();

    for row in input.iter() {
        let mut highest: u32 = 0;
        let mut lowest: u32 = 0;

        let mut i = 0;

        for col in row.iter() {
            if i == 0 {
                lowest = *col;
                highest = *col;
            }

            if *col < lowest {
                lowest = *col;
            }

            if *col > highest {
                highest = *col;
            }

            i = i + 1;
        }

        let checksum = highest - lowest;

        to_sum.push(checksum);
    }

    return sum_int_vec(to_sum);
}

pub fn two_b(input: Vec<Vec<u32>>) -> u32 {
    let mut to_sum: Vec<u32> = Vec::new();

    for row in input.iter() {
        let mut value: u32 = 0;

        let mut i = 0;
        loop {
            if i >= row.len(){
                break;
            }

            if value != 0 {
                break;
            }

            let mut j = 0;
            loop {
                if i == j {
                    j = j + 1;
                    continue;
                }

                if j >= row.len() {
                    break;
                }

                if row[i] % row[j] == 0 {                    
                    value = row[i] / row[j];
                    break;
                }

                j = j + 1;
            }

            i = i + 1;
        }

        to_sum.push(value);
    }

    return sum_int_vec(to_sum);
}

pub fn three_a(input: u32) -> u32 {
    let side_length = get_closest_odd_square_number_root(input);
    let side = get_side_from_square_number_root(side_length, input);

    return 0;
}

fn get_closest_odd_square_number_root(input: u32) -> u32 {
    let mut i: u32 = 0;

    loop {
        let square = i * i;

        if square > input {
            return i;
        }

        i += 2;
    }
}

fn get_side_from_square_number_root(root: u32, input: u32) -> SpiralSide {
    let max_value = root * root;
    let min_value = ((root - 2) * (root - 2)) + 1;

    if input == max_value || input == (max_value - root)
        || input == min_value || input == min_value + root {
        return SpiralSide::Corner;
    }

    if input < max_value && input > (max_value - root) {
        return SpiralSide::Bottom;
    }

    if input < (max_value - root) && input > (max_value - root - root) {
        return SpiralSide::Left;
    }

    if input < (max_value - root - root) && input > (max_value - (root * 3)) {
        return SpiralSide::Top;
    }

    return SpiralSide::Right;
}

enum SpiralSide {
    Top,
    Bottom,
    Left,
    Right,
    Corner
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

fn sum_int_vec(input: Vec<u32>) -> u32 {
    let mut sum: u32 = 0;

    for x in input.iter() {
        sum += x;
    }

    return sum; 
}
