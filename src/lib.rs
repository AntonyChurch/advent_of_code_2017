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
    if input == 1 {
        return 0;
    }

    let side_length = get_closest_odd_square_number_root(input);
    println!("side_length: {}", side_length);
    let side = get_side_from_square_number_root(side_length, input);

    let max_value = side_length * side_length;
    let min_value = ((side_length - 2) * (side_length - 2)) + 1;

    let bottom_right = max_value;
    let bottom_left = (max_value - side_length) + 1;
    let top_left = (max_value - side_length - side_length) + 2;
    let top_right = (max_value - side_length - side_length - side_length) + 3;

    let top_middle = (top_left + top_right) / 2;
    let left_middle = (top_left + bottom_left) / 2;
    let bottom_middle = (bottom_left + bottom_right) / 2;
    let right_middle = top_right - (side_length / 2);

    println!("top: {}, left: {}, bottom: {}, right: {}", top_middle, left_middle, bottom_middle, right_middle);

    let mut steps = (side_length - 1) / 2;
    println!("steps: {}", steps);
    match side {
        SpiralSide::Top => {
            if input > top_middle {
                println!("Top: {}", (input - top_middle));
                steps += input - top_middle;
            }
            else {
                println!("Top: {}", (top_middle - input));
                steps += top_middle - input;
            }
            
        },
        SpiralSide::Bottom => {
            if input > bottom_middle {
                println!("Bottom: {}", (input - bottom_middle));
                steps += input - bottom_middle;
            }
            else {
                println!("Bottom: {}", (bottom_middle - input));
                steps += bottom_middle - input;
            }
            
        },
        SpiralSide::Left => {
            if input > left_middle {
                println!("Left: {}", (input - left_middle));
                steps += input - left_middle;
            }
            else {
                println!("Left: {}", (left_middle - input));
                steps += left_middle - input;
            }
            
        },
        SpiralSide::Right => {
            if input > right_middle {
                println!("Right: {}", (input - right_middle));
                steps += input - right_middle;
            }
            else {
                println!("Right: {}", (right_middle - input));
                steps += right_middle - input;
            }
        },
        SpiralSide::Corner => {
            println!("Corner");
            steps = side_length - 1;
        }
    }

    return steps;
}

fn get_closest_odd_square_number_root(input: u32) -> u32 {
    let mut i: u32 = 1;

    loop {
        let square = i * i;

        if square >= input {
            return i;
        }

        i += 2;
    }
}

fn get_side_from_square_number_root(root: u32, input: u32) -> SpiralSide {
    if root == 1 {
        return SpiralSide::Corner;
    }

    let max_value = root * root;
    let min_value = ((root - 2) * (root - 2)) + 1;

    let bottom_right = max_value;
    let bottom_left = (max_value - root) + 1;
    let top_left = (max_value - root - root) + 2;
    let top_right = (max_value - root - root - root) + 3;

    println!("BR: {}, BL: {}, TL: {}, TR: {}", bottom_right, bottom_left, top_left, top_right);

    if input == bottom_right || input == bottom_left
    || input == top_left || input == top_right {
        return SpiralSide::Corner;
    }

    if input >= min_value && input < top_right {
        return SpiralSide::Right;
    }

    if input > top_right && input < top_left {
        return SpiralSide::Top;
    }

    if input > top_left && input < bottom_left {
        return SpiralSide::Left;
    }
    
    if input > bottom_left && input < bottom_right {
        return SpiralSide::Bottom;
    }

    return SpiralSide::Corner;
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
