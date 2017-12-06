extern crate adventofcode;

#[test]
fn example_a_one(){        
    let mut veccy: Vec<Vec<u32>> = Vec::new();

    let row1: Vec<u32> = vec![5, 1, 9, 5];
    let row2: Vec<u32> = vec![7, 5, 3];
    let row3: Vec<u32> = vec![2, 4, 6, 8];

    veccy.push(row1);
    veccy.push(row2);
    veccy.push(row3);

    assert_eq!(18, adventofcode::two_a(veccy));
}

#[test]
fn example_b_one(){
    let mut veccy: Vec<Vec<u32>> = Vec::new();

    let row1: Vec<u32> = vec![5, 9, 2, 8];
    let row2: Vec<u32> = vec![9, 4, 7, 3];
    let row3: Vec<u32> = vec![3, 8, 6, 5];

    veccy.push(row1);
    veccy.push(row2);
    veccy.push(row3);

    assert_eq!(9, adventofcode::two_b(veccy));
}