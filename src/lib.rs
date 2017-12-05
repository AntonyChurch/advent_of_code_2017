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