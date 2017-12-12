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

    let mut steps = (side_length - 1) / 2;
    match side {
        SpiralSide::Top => {
            if input > top_middle {
                steps += input - top_middle;
            }
            else {
                steps += top_middle - input;
            }
            
        },
        SpiralSide::Bottom => {
            if input > bottom_middle {
                steps += input - bottom_middle;
            }
            else {
                steps += bottom_middle - input;
            }
            
        },
        SpiralSide::Left => {
            if input > left_middle {
                steps += input - left_middle;
            }
            else {
                steps += left_middle - input;
            }
            
        },
        SpiralSide::Right => {
            if input > right_middle {
                steps += input - right_middle;
            }
            else {
                steps += right_middle - input;
            }
        },
        SpiralSide::Corner => {
            steps = side_length - 1;
        }
    }

    return steps;
}

pub fn three_b(input: u32) -> u32 {
    // Create data struct { x, y, value}
    // Add {0, 0, 1} to a list
    // Start to calculate those around it
    // Return answer

    let mut values: Vec<SpiralPositionValue> = Vec::new();
    values.push(SpiralPositionValue{ position: SpiralPosition{ x: 0, y: 0 }, value: 1 });

    let mut step = 1;
    let mut last_x = 1;
    let mut last_y = 0;

    loop {
        while last_y < step {
            let pos_value = get_value_for_position(last_x, last_y, &values);
            if pos_value > input {
                return pos_value;
            }

            values.push(SpiralPositionValue{ position: SpiralPosition {x: last_x, y: last_y}, value: pos_value });

            last_y += 1;
        }

        while last_x > -step {
            let pos_value = get_value_for_position(last_x, last_y, &values);
            if pos_value > input {
                return pos_value;
            }

            values.push(SpiralPositionValue{ position: SpiralPosition {x: last_x, y: last_y}, value: pos_value });

            last_x -= 1;
        }

        while last_y > -step  {
            let pos_value = get_value_for_position(last_x, last_y, &values);
            if pos_value > input {
                return pos_value;
            }

            values.push(SpiralPositionValue{ position: SpiralPosition {x: last_x, y: last_y}, value: pos_value });

            last_y -= 1;
        }

        while last_x <= step {
            let pos_value = get_value_for_position(last_x, last_y, &values);
            if pos_value > input {
                return pos_value;
            }

            values.push(SpiralPositionValue{ position: SpiralPosition {x: last_x, y: last_y}, value: pos_value });

            last_x += 1;
        }

        step += 1;
    }

    return 0;
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

fn get_value_for_position(x: i32, y: i32, values: &[SpiralPositionValue]) -> u32 {
    let mut value = 0;
    value += get_value_from_position(x - 1, y + 1, values);
    value += get_value_from_position(x, y + 1, values);
    value += get_value_from_position(x + 1, y + 1, values);
    value += get_value_from_position(x + 1, y, values);
    value += get_value_from_position(x + 1, y - 1, values);
    value += get_value_from_position(x, y - 1, values);
    value +=  get_value_from_position(x - 1, y - 1, values);
    value += get_value_from_position(x - 1, y, values);

    println!("x: {}, y: {}, val: {}", x, y, value);
    
    return value;
}

fn get_value_from_position(x: i32, y: i32, values: &[SpiralPositionValue]) -> u32 {
    let mut ret_value = 0;

    // For problem 3B, counting backwards would probably be much more efficient.
    for val in values {
        if val.position.x == x && val.position.y == y {
            ret_value = val.value;
            break;
        }
    }

    return ret_value;
}

enum SpiralSide {
    Top,
    Bottom,
    Left,
    Right,
    Corner
}

struct SpiralPositionValue{
    position: SpiralPosition,
    value: u32,
}

struct SpiralPosition{
    x: i32,
    y: i32
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
